//! GGA_X_LAG vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`
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
pub fn gga_x_lag_vxc_unpol(
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
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t7 = ((t4).select(t5, (t4).select(-t5, f64x8::splat(0.0))));
            let t8 = f64x8::splat(1.0) + t7;
            let t10 = (simd::cbrt(zeta_threshold));
            let t12 = (simd::cbrt(t8));
            let t14 = (((t8).simd_le(zeta_threshold)).select(t10 * zeta_threshold, t12 * t8));
            let t15 = t3 * t14;
            let t16 = (simd::cbrt(v_rho));
            let t17 = f64x8::splat(M_CBRT6);
            let t18 = t17 * t17;
            let t19 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t20 = (simd::cbrt(t19));
            let t21 = f64x8::splat(1.0) / t20;
            let t22 = t18 * t21;
            let t23 = ((v_sigma).sqrt());
            let t24 = f64x8::splat(M_CBRT2);
            let t29 = t22 * t23 * t24 / t16 / v_rho;
            let t30 = (simd::pow(t29, f64x8::splat(2.626712)));
            let t33 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t30;
            let t34 = (simd::pow(t33, -f64x8::splat(0.657946)));
            let t38 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t15 * t16 * t30 * t34));
            let tzk0 = f64x8::splat(2.0) * t38;
            acc_zk = tzk0;
            let t39 = t16 * t16;
            let t45 = v_rho * v_rho;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = (simd::pow(t29, f64x8::splat(1.626712)));
            let t49 = t15 * t46 * t47;
            let t50 = t34 * t18;
            let t52 = t21 * t23 * t24;
            let t53 = t50 * t52;
            let t56 = (simd::pow(t29, f64x8::splat(4.253424)));
            let t58 = t15 * t46 * t56;
            let t59 = (simd::pow(t33, -f64x8::splat(1.657946)));
            let t60 = t59 * t18;
            let t61 = t60 * t52;
            let t65 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.133342923975857e-06) * t15 / t39 * t30 * t34 + f64x8::splat(5.393525383408988e-05) * t49 * t53 - f64x8::splat(4.780604235623332e-09) * t58 * t61));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t65 + f64x8::splat(2.0) * t38;
            acc_vrho = tvrho0;
            let t68 = f64x8::splat(1.0) / v_rho;
            let t70 = t15 * t68 * t47;
            let t71 = f64x8::splat(1.0) / t23;
            let t73 = t21 * t71 * t24;
            let t74 = t50 * t73;
            let t78 = t15 * t68 * t56;
            let t79 = t60 * t73;
            let t83 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0225720187783704e-05) * t70 * t74 + f64x8::splat(1.7927265883587494e-09) * t78 * t79));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t83;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
