//! GGA_K_EXP4 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`
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
pub fn gga_k_exp4_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t26;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t24 * t28;
            let t30 = f64x8::splat(M_CBRT2);
            let t31 = t30 * t30;
            let t32 = v_sigma * t31;
            let t33 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t22 / t33;
            let t39 = (simd::exp(-f64x8::splat(8.325416666666667) * t29 * t32 * t35));
            let t41 = t24 * t24;
            let t43 = f64x8::splat(1.0) / t26 / t25;
            let t44 = t41 * t43;
            let t45 = v_sigma * v_sigma;
            let t47 = t33 * t33;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t21 / t48;
            let t54 = (simd::exp(-f64x8::splat(0.015095833333333333) * t44 * t45 * t30 * t50));
            let t56 = f64x8::splat(2.0788) - f64x8::splat(0.8524) * t39 - f64x8::splat(1.2264) * t54;
            let t60 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t56));
            let tzk0 = f64x8::splat(2.0) * t60;
            acc_zk = tzk0;
            let t62 = t20 / t21;
            let t66 = t29 * v_sigma;
            let t67 = t33 * v_rho;
            let t71 = t31 / t22 / t67 * t39;
            let t74 = t44 * t45;
            let t75 = t47 * t33;
            let t77 = f64x8::splat(1.0) / t21 / t75;
            let t78 = t30 * t77;
            let t79 = t78 * t54;
            let t82 = -f64x8::splat(18.92422711111111) * t66 * t71 - f64x8::splat(0.09873882666666667) * t74 * t79;
            let t87 = ((t2).select(f64x8::splat(0.0), t7 * t62 * t56 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t82));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t87 + f64x8::splat(2.0) * t60;
            acc_vrho = tvrho0;
            let t94 = t44 * v_sigma;
            let t95 = t30 * t50;
            let t96 = t95 * t54;
            let t99 = f64x8::splat(7.096585166666666) * t29 * t31 * t35 * t39 + f64x8::splat(0.03702706) * t94 * t96;
            let t103 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t99));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t103;
            acc_vsigma = tvsigma0;
            let t108 = t20 / t21 / v_rho;
            let t118 = t31 / t22 / t47 * t39;
            let t121 = t47 * t67;
            let t123 = f64x8::splat(1.0) / t21 / t121;
            let t124 = t30 * t123;
            let t125 = t124 * t39;
            let t128 = t124 * t54;
            let t131 = t25 * t25;
            let t134 = t24 / t27 / t131;
            let t135 = t45 * t45;
            let t136 = t134 * t135;
            let t137 = t47 * t47;
            let t138 = t137 * t47;
            let t140 = f64x8::splat(1.0) / t22 / t138;
            let t142 = t31 * t140 * t54;
            let t145 = f64x8::splat(69.38883274074074) * t66 * t118 - f64x8::splat(840.277737571358) * t74 * t125 + f64x8::splat(0.6253459022222222) * t74 * t128 - f64x8::splat(0.047697435868444445) * t136 * t142;
            let t150 = ((t2).select(f64x8::splat(0.0), -t7 * t108 * t56 / f64x8::splat(30.0) + t7 * t62 * t82 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t145));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t150 + f64x8::splat(4.0) * t87;
            acc_v2rho2 = tv2rho20;
            let t158 = t44 * t30;
            let t165 = t45 * v_sigma;
            let t166 = t134 * t165;
            let t167 = t137 * t67;
            let t169 = f64x8::splat(1.0) / t22 / t167;
            let t174 = -f64x8::splat(18.92422711111111) * t29 * t71 + f64x8::splat(315.10415158925923) * t158 * t77 * v_sigma * t39 - f64x8::splat(0.19747765333333334) * t94 * t79 + f64x8::splat(0.017886538450666668) * t166 * t31 * t169 * t54;
            let t179 = ((t2).select(f64x8::splat(0.0), t7 * t62 * t99 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t174));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t179 + f64x8::splat(2.0) * t103;
            acc_v2rhosigma = tv2rhosigma0;
            let t188 = t137 * t33;
            let t190 = f64x8::splat(1.0) / t22 / t188;
            let t192 = t31 * t190 * t54;
            let t195 = -f64x8::splat(118.16405684597223) * t44 * t95 * t39 + f64x8::splat(0.03702706) * t44 * t96 - f64x8::splat(0.006707451919) * t134 * t45 * t192;
            let t199 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t195));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t199;
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
