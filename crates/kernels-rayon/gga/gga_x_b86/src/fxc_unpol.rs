//! GGA_X_B86 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`
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
pub fn gga_x_b86_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
    let param_omega = f64x8::splat(param_omega);
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = param_beta * v_sigma;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t27 = t22 * t26;
            let t30 = param_gamma * v_sigma * t27 + f64x8::splat(1.0);
            let t31 = (simd::pow(t30, param_omega));
            let t32 = f64x8::splat(1.0) / t31;
            let t35 = t20 * t27 * t32 + f64x8::splat(1.0);
            let t39 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t35));
            let tzk0 = f64x8::splat(2.0) * t39;
            acc_zk = tzk0;
            let t41 = t17 / t24;
            let t45 = t23 * v_rho;
            let t47 = f64x8::splat(1.0) / t24 / t45;
            let t52 = v_sigma * v_sigma;
            let t53 = param_beta * t52;
            let t54 = t23 * t23;
            let t55 = t54 * t23;
            let t57 = f64x8::splat(1.0) / t18 / t55;
            let t60 = t32 * param_omega;
            let t61 = f64x8::splat(1.0) / t30;
            let t63 = t60 * param_gamma * t61;
            let t66 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t20 * t22 * t47 * t32 + f64x8::splat(16.0) / f64x8::splat(3.0) * t53 * t21 * t57 * t63;
            let t71 = ((t2).select(f64x8::splat(0.0), -t6 * t41 * t35 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t66));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t71 + f64x8::splat(2.0) * t39;
            acc_vrho = tvrho0;
            let t74 = param_beta * t22;
            let t77 = t54 * v_rho;
            let t79 = f64x8::splat(1.0) / t18 / t77;
            let t84 = -f64x8::splat(2.0) * t20 * t21 * t79 * t63 + t74 * t26 * t32;
            let t88 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t84));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t88;
            acc_vsigma = tvsigma0;
            let t93 = t17 / t24 / v_rho;
            let t101 = f64x8::splat(1.0) / t24 / t54;
            let t106 = t54 * t45;
            let t108 = f64x8::splat(1.0) / t18 / t106;
            let t113 = t52 * v_sigma;
            let t114 = param_beta * t113;
            let t115 = t54 * t54;
            let t116 = t115 * t23;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t114 * t117;
            let t119 = param_omega * param_omega;
            let t120 = t32 * t119;
            let t121 = param_gamma * param_gamma;
            let t122 = t30 * t30;
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t121 * t123;
            let t125 = t120 * t124;
            let t128 = t60 * t124;
            let t131 = f64x8::splat(88.0) / f64x8::splat(9.0) * t20 * t22 * t101 * t32 - f64x8::splat(48.0) * t53 * t21 * t108 * t63 + f64x8::splat(256.0) / f64x8::splat(9.0) * t118 * t125 + f64x8::splat(256.0) / f64x8::splat(9.0) * t118 * t128;
            let t136 = ((t2).select(f64x8::splat(0.0), t6 * t93 * t35 / f64x8::splat(12.0) - t6 * t41 * t66 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t131));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t136 + f64x8::splat(4.0) * t71;
            acc_v2rho2 = tv2rho20;
            let t145 = param_beta * t21;
            let t150 = param_omega * param_gamma * v_sigma * t61;
            let t153 = t115 * v_rho;
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t53 * t154;
            let t160 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t74 * t47 * t32 + f64x8::splat(16.0) * t145 * t57 * t32 * t150 - f64x8::splat(32.0) / f64x8::splat(3.0) * t155 * t125 - f64x8::splat(32.0) / f64x8::splat(3.0) * t155 * t128;
            let t165 = ((t2).select(f64x8::splat(0.0), -t6 * t41 * t84 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t160));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t165 + f64x8::splat(2.0) * t88;
            acc_v2rhosigma = tv2rhosigma0;
            let t170 = f64x8::splat(1.0) / t115;
            let t171 = t20 * t170;
            let t175 = -f64x8::splat(4.0) * t145 * t79 * t63 + f64x8::splat(4.0) * t171 * t125 + f64x8::splat(4.0) * t171 * t128;
            let t179 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t175));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t179;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
