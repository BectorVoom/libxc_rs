//! B-spline evaluation for `hyb_gga_xc_case21` (k = 3, Nsp = 10).
//!
//! A line-for-line port of libxc's `xc_bspline` (`util.c`) and of the knot
//! sequence `case21_set_ext_params` (`hyb_gga_xc_case21.c`) builds.
//! Bit-exactness against libxc needs both to be literal:
//!
//! * the knots are `qmin + k*dq` evaluated in floating point, not the exact
//!   `(k - 3)/7`: the last knot comes out `0x1.6db6db6db6db6p+0`, one ulp
//!   below the correctly rounded 10/7;
//! * the Cox-de Boor table uses libxc's `saved` recurrence and its derivative
//!   table, in libxc's operation order. An algebraically equal closed form
//!   rounds differently: the unrolled k = 3 version this replaces was 1-2 ulp
//!   off on `zk` and up to 3e-14 relative on `vsigma` (2026-09-11).

/// Spline order `k` (`params->k`).
const K: usize = 3;
/// Number of B-splines (`params->Nsp`).
const NSP: usize = 10;
/// `nknots = Nsp + k + 1`.
const NKNOTS: usize = NSP + K + 1;

/// `case21_set_ext_params`:
///
/// ```c
/// double qmin = -params->k*1.0/(params->Nsp - params->k);
/// double qmax = params->Nsp*1.0/(params->Nsp - params->k);
/// double dq  = (qmax - qmin)/(nknots-1);
/// params->knots[k] = qmin+k*dq;
/// ```
///
/// Const evaluation rounds IEEE round-to-nearest, as the runtime C does.
const KNOTS: [f64; NKNOTS] = {
    let qmin = -(K as f64) * 1.0 / (NSP - K) as f64;
    let qmax = NSP as f64 * 1.0 / (NSP - K) as f64;
    let dq = (qmax - qmin) / (NKNOTS - 1) as f64;
    let mut u = [0.0; NKNOTS];
    let mut k = 0;
    while k < NKNOTS {
        u[k] = qmin + k as f64 * dq;
        k += 1;
    }
    u
};

/// libxc `PMAX`: the dense table's static size.
const PMAX: usize = 8;

/// `xc_bspline(i, p, u, nderiv, U, ders)`: the `i`-th B-spline of degree `p`
/// on knots `uk` and its derivatives up to `nderiv`, into `ders[0..=nderiv]`.
fn xc_bspline(i: usize, p: usize, u: f64, nderiv: usize, uk: &[f64], ders: &mut [f64; 5]) {
    // Initialize output array
    for d in ders.iter_mut().take(nderiv + 1) {
        *d = 0.0;
    }

    // Check locality of support
    if u < uk[i] || u >= uk[i + p + 1] {
        return;
    }

    // Array of normalized B splines, dense storage
    let mut n = [[0.0f64; PMAX]; PMAX];

    // Zeroth-degree functions: piecewise constants
    for j in 0..=p {
        n[0][j] = if u >= uk[i + j] && u < uk[i + j + 1] {
            1.0
        } else {
            0.0
        };
    }

    // Table of B splines
    for k in 1..=p {
        let mut saved = if n[k - 1][0] == 0.0 {
            0.0
        } else {
            ((u - uk[i]) * n[k - 1][0]) / (uk[i + k] - uk[i])
        };
        for j in 0..=(p - k) {
            let ul = uk[i + j + 1];
            let ur = uk[i + j + k + 1];
            if n[k - 1][j + 1] == 0.0 {
                n[k][j] = saved;
                saved = 0.0;
            } else {
                let temp = n[k - 1][j + 1] / (ur - ul);
                n[k][j] = saved + (ur - u) * temp;
                saved = (u - ul) * temp;
            }
        }
    }

    // Function value
    ders[0] = n[p][0];
    if nderiv == 0 {
        return;
    }

    // Derivatives
    let mut nd = [0.0f64; 5];
    let maxk = if nderiv < p { nderiv } else { p };
    for k in 1..=maxk {
        // Load appropriate column
        for v in nd.iter_mut().take(nderiv + 1) {
            *v = 0.0;
        }
        for j in 0..=k {
            nd[j] = n[p - k][j];
        }

        // Compute table
        for jj in 1..=k {
            let mut saved = if nd[0] == 0.0 {
                0.0
            } else {
                nd[0] / (uk[i + p - k + jj] - uk[i])
            };
            for j in 0..=(k - jj) {
                let ul = uk[i + j + 1];
                // the -k term is missing in the book
                let ur = uk[i + j + p - k + jj + 1];
                let m = (p - k + jj) as f64;
                if nd[j + 1] == 0.0 {
                    nd[j] = m * saved;
                    saved = 0.0;
                } else {
                    let temp = nd[j + 1] / (ur - ul);
                    nd[j] = m * (saved - temp);
                    saved = temp;
                }
            }
        }
        // k:th derivative
        ders[k] = nd[0];
    }
}

/// `xbspline` / `cbspline` in `hyb_gga_xc_case21.c`:
///
/// ```c
/// double result=0.0;
/// for(int i=0;i<params->Nsp;i++) {
///   xc_bspline(i, params->k, u, ider, params->knots, temp);
///   result += params->cx[i]*temp[ider];
/// }
/// ```
fn spline_sum(u: f64, ider: f64, c: &[f64; NSP]) -> f64 {
    let ider = ider as usize;
    assert!(ider <= 4);
    let mut temp = [0.0f64; 5];
    let mut result = 0.0;
    for (i, ci) in c.iter().enumerate() {
        xc_bspline(i, K, u, ider, &KNOTS, &mut temp);
        result += ci * temp[ider];
    }
    result
}

/// case21 exchange enhancement: `sum_i cx[i] * B_{i,3}^{(ider)}(u)`.
#[allow(clippy::too_many_arguments)]
pub fn case21_xbspline(
    u: f64,
    ider: f64,
    cx_0: f64,
    cx_1: f64,
    cx_2: f64,
    cx_3: f64,
    cx_4: f64,
    cx_5: f64,
    cx_6: f64,
    cx_7: f64,
    cx_8: f64,
    cx_9: f64,
) -> f64 {
    spline_sum(
        u,
        ider,
        &[cx_0, cx_1, cx_2, cx_3, cx_4, cx_5, cx_6, cx_7, cx_8, cx_9],
    )
}

/// case21 correlation enhancement: `sum_i cc[i] * B_{i,3}^{(ider)}(u)`.
#[allow(clippy::too_many_arguments)]
pub fn case21_cbspline(
    u: f64,
    ider: f64,
    cc_0: f64,
    cc_1: f64,
    cc_2: f64,
    cc_3: f64,
    cc_4: f64,
    cc_5: f64,
    cc_6: f64,
    cc_7: f64,
    cc_8: f64,
    cc_9: f64,
) -> f64 {
    spline_sum(
        u,
        ider,
        &[cc_0, cc_1, cc_2, cc_3, cc_4, cc_5, cc_6, cc_7, cc_8, cc_9],
    )
}
