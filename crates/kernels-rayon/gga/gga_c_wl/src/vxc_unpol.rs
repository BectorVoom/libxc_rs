//! GGA_C_WL vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`
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
pub fn gga_c_wl_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t1 = ((v_sigma).sqrt());
            let t2 = (simd::cbrt(v_rho));
            let t4 = f64x8::splat(1.0) / t2 / v_rho;
            let t5 = t1 * t4;
            let t7 = -f64x8::splat(0.7486) + f64x8::splat(0.06001) * t5;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t1 * t8;
            let t12 = f64x8::splat(M_CBRT3);
            let t14 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t15 = t12 * t14;
            let t16 = f64x8::splat(M_CBRT4);
            let t17 = t16 * t16;
            let t18 = f64x8::splat(1.0) / t2;
            let t22 = f64x8::splat(3.60073) + f64x8::splat(1.8) * t9 * t4 + t15 * t17 * t18 / f64x8::splat(4.0);
            let t23 = f64x8::splat(1.0) / t22;
            let tzk0 = t7 * t23;
            acc_zk = tzk0;
            let t26 = v_rho * t7;
            let t27 = t22 * t22;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t2 / t29;
            let t37 = -f64x8::splat(2.4) * t9 * t31 - t15 * t17 * t4 / f64x8::splat(12.0);
            let t38 = t28 * t37;
            let tvrho0 = tzk0 - f64x8::splat(0.08001333333333334) * t5 * t23 - t26 * t38;
            acc_vrho = tvrho0;
            let t40 = f64x8::splat(1.0) / t1;
            let t41 = t18 * t40;
            let t44 = t18 * t7;
            let t46 = t28 * t40 * t8;
            let tvsigma0 = f64x8::splat(0.030005) * t41 * t23 - f64x8::splat(0.9) * t44 * t46;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
