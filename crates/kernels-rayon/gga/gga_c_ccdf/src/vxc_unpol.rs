//! GGA_C_CCDF vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_ccdf_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_c3 = f64x8::splat(param_c3);
    let param_c4 = f64x8::splat(param_c4);
    let param_c5 = f64x8::splat(param_c5);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t1 = (simd::cbrt(v_rho));
            let t2 = f64x8::splat(1.0) / t1;
            let t4 = param_c2 * t2 + f64x8::splat(1.0);
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = param_c1 * t5;
            let t7 = f64x8::splat(M_CBRT2);
            let t8 = f64x8::splat(M_CBRT6);
            let t9 = t8 * t8;
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = f64x8::splat(1.0) / t12;
            let t14 = ((v_sigma).sqrt());
            let t15 = t13 * t14;
            let t17 = f64x8::splat(1.0) / t1 / v_rho;
            let t23 = (simd::exp(-param_c4 * (t10 * t15 * t17 / f64x8::splat(12.0) - param_c5)));
            let t24 = f64x8::splat(1.0) + t23;
            let t27 = f64x8::splat(1.0) - param_c3 / t24;
            let tzk0 = t6 * t27;
            acc_zk = tzk0;
            let t28 = t2 * param_c1;
            let t29 = t4 * t4;
            let t30 = f64x8::splat(1.0) / t29;
            let t36 = t5 * param_c3;
            let t37 = t24 * t24;
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t36 * t38;
            let t40 = t17 * param_c1 * t39;
            let t42 = param_c4 * t7 * t9;
            let tvrho0 = tzk0 + t28 * t30 * t27 * param_c2 / f64x8::splat(3.0) + t40 * t42 * t15 * t23 / f64x8::splat(9.0);
            acc_vrho = tvrho0;
            let t47 = t28 * t39;
            let t48 = f64x8::splat(1.0) / t14;
            let t51 = t42 * t13 * t48 * t23;
            let tvsigma0 = -t47 * t51 / f64x8::splat(24.0);
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
