//! GGA_C_LM fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`
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
pub fn gga_c_lm_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_lm_f = f64x8::splat(param_lm_f);
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
            let t1 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t2 = f64x8::splat(1.0) / v_rho;
            let t5 = f64x8::splat(1.0) + t1 * t2 / f64x8::splat(36000.0);
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(t1));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = (simd::cbrt(v_rho));
            let t14 = t10 * t11 * t12;
            let t16 = f64x8::splat(1.0) + f64x8::splat(10.0) * t14;
            let t17 = (simd::ln(t16));
            let t19 = f64x8::splat(0.0252) * t5 * t17;
            let t20 = t8 * t8;
            let t21 = t7 * t20;
            let t22 = t12 * t12;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = t11 * t23;
            let t25 = t21 * t24;
            let t26 = f64x8::splat(7e-06) * t25;
            let t27 = t6 * t8;
            let t28 = t11 * t11;
            let t31 = t27 * t28 / t12;
            let t32 = f64x8::splat(0.000105) * t31;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(5.658842421045167e-07) * t2;
            let t47 = f64x8::splat(1.0) + f64x8::splat(25.0) * t14;
            let t48 = (simd::ln(t47));
            let t54 = t43 * (-f64x8::splat(0.0127) * t45 * t48 - f64x8::splat(6.435555555555556e-06) * t25 + f64x8::splat(8.383333333333333e-05) * t31 - f64x8::splat(0.004166666666666667) + t19);
            let t55 = f64x8::splat(M_PI) * t7;
            let t56 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t57 = (simd::cbrt(t56));
            let t59 = f64x8::splat(1.0) / t57 / t56;
            let t60 = v_rho * v_rho;
            let t62 = f64x8::splat(1.0) / t22 / t60;
            let t63 = v_sigma * t62;
            let t66 = t34 * t34;
            let t68 = ((t33).select(t66 * zeta_threshold, f64x8::splat(1.0)));
            let t69 = ((t68).sqrt());
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::pow(t1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = ((v_sigma).sqrt());
            let t75 = t73 * t74;
            let t76 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t81 = (simd::exp(-t6 * param_lm_f * t75 / t76 / v_rho));
            let t82 = t70 * t81;
            let t86 = t59 * (-f64x8::splat(7.0) / f64x8::splat(9.0) * t63 * t36 + f64x8::splat(2.0) * t82 * t63);
            let t89 = t55 * t86 * t12 / f64x8::splat(144.0);
            let tzk0 = -t19 + t26 - t32 + f64x8::splat(0.0084) + t54 + t89;
            acc_zk = tzk0;
            let t90 = f64x8::splat(1.0) / t60;
            let t92 = t1 * t90 * t17;
            let t93 = f64x8::splat(7e-07) * t92;
            let t95 = t5 * t7 * t9;
            let t96 = f64x8::splat(1.0) / t16;
            let t98 = t95 * t24 * t96;
            let t99 = f64x8::splat(0.084) * t98;
            let t101 = f64x8::splat(1.0) / t22 / v_rho;
            let t102 = t11 * t101;
            let t103 = t21 * t102;
            let t105 = t12 * v_rho;
            let t107 = t28 / t105;
            let t108 = t27 * t107;
            let t113 = t45 * t7 * t9;
            let t114 = f64x8::splat(1.0) / t47;
            let t121 = t43 * (f64x8::splat(7.1867298747273625e-09) * t90 * t48 - f64x8::splat(0.10583333333333333) * t113 * t24 * t114 + f64x8::splat(4.290370370370371e-06) * t103 - f64x8::splat(2.7944444444444445e-05) * t108 - t93 + t99);
            let t122 = t60 * v_rho;
            let t124 = f64x8::splat(1.0) / t22 / t122;
            let t125 = v_sigma * t124;
            let t129 = t70 * t6 * param_lm_f;
            let t130 = t74 * v_sigma;
            let t131 = t73 * t130;
            let t132 = t60 * t60;
            let t133 = t76 * t76;
            let t134 = t133 * t133;
            let t135 = t134 * t76;
            let t138 = f64x8::splat(1.0) / t135 / t132 * t81;
            let t145 = t59 * (f64x8::splat(56.0) / f64x8::splat(27.0) * t125 * t36 + f64x8::splat(7.0) / f64x8::splat(3.0) * t129 * t131 * t138 - f64x8::splat(16.0) / f64x8::splat(3.0) * t82 * t125);
            let t147 = t55 * t145 * t12;
            let t150 = t55 * t86 * t23;
            let tvrho0 = -t19 + t26 - t32 + f64x8::splat(0.0084) + t54 + t89 + v_rho * (t93 - t99 - f64x8::splat(4.666666666666666e-06) * t103 + f64x8::splat(3.5e-05) * t108 + t121 + t147 / f64x8::splat(144.0) + t150 / f64x8::splat(432.0));
            acc_vrho = tvrho0;
            let t154 = t105 * f64x8::splat(M_PI);
            let t155 = t7 * t59;
            let t160 = f64x8::splat(1.0) / t135 / t122 * t81;
            let t165 = -f64x8::splat(7.0) / f64x8::splat(9.0) * t62 * t36 - t129 * t75 * t160 + f64x8::splat(2.0) * t82 * t62;
            let tvsigma0 = t154 * t155 * t165 / f64x8::splat(144.0);
            acc_vsigma = tvsigma0;
            let t175 = f64x8::splat(1.0) / t122;
            let t177 = t1 * t175 * t17;
            let t178 = f64x8::splat(1.4e-06) * t177;
            let t181 = t9 * t11;
            let t182 = t181 * t96;
            let t183 = t1 * t62 * t7 * t182;
            let t184 = f64x8::splat(4.666666666666666e-06) * t183;
            let t186 = t95 * t102 * t96;
            let t187 = f64x8::splat(0.056) * t186;
            let t189 = f64x8::splat(1.0) / t20;
            let t190 = t5 * t6 * t189;
            let t191 = t16 * t16;
            let t192 = f64x8::splat(1.0) / t191;
            let t194 = t190 * t107 * t192;
            let t195 = f64x8::splat(0.84) * t194;
            let t196 = t11 * t62;
            let t197 = t21 * t196;
            let t201 = t28 / t12 / t60;
            let t202 = t27 * t201;
            let t207 = t181 * t114;
            let t214 = t45 * t6 * t189;
            let t215 = t47 * t47;
            let t216 = f64x8::splat(1.0) / t215;
            let t223 = t43 * (-f64x8::splat(1.4373459749454725e-08) * t175 * t48 + f64x8::splat(1.1977883124545604e-07) * t62 * t7 * t207 + f64x8::splat(0.07055555555555555) * t113 * t102 * t114 + f64x8::splat(2.6458333333333335) * t214 * t107 * t216 - f64x8::splat(7.150617283950617e-06) * t197 + f64x8::splat(3.725925925925926e-05) * t202 + t178 - t184 - t187 - t195);
            let t225 = f64x8::splat(1.0) / t22 / t132;
            let t226 = v_sigma * t225;
            let t229 = t132 * v_rho;
            let t232 = f64x8::splat(1.0) / t135 / t229 * t81;
            let t237 = param_lm_f * param_lm_f;
            let t238 = t70 * t7 * t237;
            let t239 = v_sigma * v_sigma;
            let t240 = t9 * t239;
            let t241 = t132 * t122;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t242 * t81;
            let t250 = t59 * (-f64x8::splat(616.0) / f64x8::splat(81.0) * t226 * t36 - f64x8::splat(35.0) / f64x8::splat(2.0) * t129 * t131 * t232 + f64x8::splat(49.0) / f64x8::splat(18.0) * t238 * t240 * t243 + f64x8::splat(176.0) / f64x8::splat(9.0) * t82 * t226);
            let t252 = t55 * t250 * t12;
            let t255 = t55 * t145 * t23;
            let t258 = t55 * t86 * t101;
            let tv2rho20 = f64x8::splat(1.4e-06) * t92 - f64x8::splat(0.168) * t98 - f64x8::splat(9.333333333333333e-06) * t103 + f64x8::splat(7e-05) * t108 + f64x8::splat(2.0) * t121 + t147 / f64x8::splat(72.0) + t150 / f64x8::splat(216.0) + v_rho * (-t178 + t184 + t187 + t195 + f64x8::splat(7.777777777777777e-06) * t197 - f64x8::splat(4.6666666666666665e-05) * t202 + t223 + t252 / f64x8::splat(144.0) + t255 / f64x8::splat(216.0) - t258 / f64x8::splat(648.0));
            acc_v2rho2 = tv2rho20;
            let t262 = t59 * t165;
            let t271 = t9 * v_sigma;
            let t272 = t132 * t60;
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t273 * t81;
            let t280 = f64x8::splat(56.0) / f64x8::splat(27.0) * t124 * t36 + f64x8::splat(37.0) / f64x8::splat(6.0) * t129 * t75 * t138 - f64x8::splat(7.0) / f64x8::splat(6.0) * t238 * t271 * t274 - f64x8::splat(16.0) / f64x8::splat(3.0) * t82 * t124;
            let tv2rhosigma0 = t55 * t262 * t12 / f64x8::splat(108.0) + t154 * t155 * t280 / f64x8::splat(144.0);
            acc_v2rhosigma = tv2rhosigma0;
            let t284 = f64x8::splat(1.0) / t74;
            let t285 = t73 * t284;
            let t289 = f64x8::splat(1.0) / t229;
            let t295 = t155 * (-f64x8::splat(3.0) / f64x8::splat(2.0) * t129 * t285 * t160 + t238 * t9 * t289 * t81 / f64x8::splat(2.0));
            let tv2sigma20 = t154 * t295 / f64x8::splat(144.0);
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
