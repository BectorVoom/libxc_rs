//! HYB_MGGA_XC_GAS22 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_xc_gas22.c`
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
pub fn hyb_mgga_xc_gas22_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_0: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
    param_c_os_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_x_1 = f64x8::splat(param_c_x_1);
    let param_c_x_2 = f64x8::splat(param_c_x_2);
    let param_c_x_0 = f64x8::splat(param_c_x_0);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_os_1 = f64x8::splat(param_c_os_1);
    let param_c_os_2 = f64x8::splat(param_c_os_2);
    let param_c_os_3 = f64x8::splat(param_c_os_3);
    let param_c_os_4 = f64x8::splat(param_c_os_4);
    let param_c_os_0 = f64x8::splat(param_c_os_0);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t4);
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t8 = (simd::cbrt(t7));
            let t9 = t6 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t10 * t10;
            let t12 = f64x8::splat(M_CBRT2);
            let t14 = t9 * t11 * t12;
            let t15 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t19 = ((t15).select(t17, f64x8::splat(2.0) * t12));
            let t20 = (simd::cbrt(v_rho));
            let t21 = t19 * t20;
            let t22 = (simd::cbrt(f64x8::splat(9.0)));
            let t23 = t22 * t22;
            let t24 = t8 * t8;
            let t26 = t23 * t24 * param_hyb_omega_0;
            let t27 = f64x8::splat(1.0) / t20;
            let t29 = ((t15).select(t16, t12));
            let t31 = t12 / t29;
            let t34 = t26 * t6 * t27 * t31 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.35)).simd_le(t34);
            let t36 = (f64x8::splat(1.35)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.35)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = f64x8::splat(1.0) / t47 / t38;
            let t54 = f64x8::splat(1.0) / t47 / t41;
            let t57 = f64x8::splat(1.0) / t47 / t44;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = ((t36).select(f64x8::splat(1.35), t34));
            let t64 = ((f64x8::splat(M_PI)).sqrt());
            let t65 = f64x8::splat(1.0) / t63;
            let t67 = (simd::erf(t65 / f64x8::splat(2.0)));
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::exp(-t70 / f64x8::splat(4.0)));
            let t73 = t72 - f64x8::splat(1.0);
            let t76 = t72 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t69 * t73;
            let t79 = f64x8::splat(2.0) * t63 * t76 + t64 * t67;
            let t83 = ((t35).select(f64x8::splat(1.0) / t38 / f64x8::splat(36.0) - t42 / f64x8::splat(960.0) + t45 / f64x8::splat(26880.0) - t48 / f64x8::splat(829440.0) + t51 / f64x8::splat(28385280.0) - t54 / f64x8::splat(1073479680.0) + t57 / f64x8::splat(44590694400.0) - t60 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t79));
            let t85 = param_c_x_1;
            let t86 = t85 * v_sigma;
            let t87 = t12 * t12;
            let t88 = v_rho * v_rho;
            let t89 = t20 * t20;
            let t91 = f64x8::splat(1.0) / t89 / t88;
            let t92 = t87 * t91;
            let t93 = v_sigma * t87;
            let t94 = t93 * t91;
            let t96 = f64x8::splat(1.0) + f64x8::splat(0.003840616724010807) * t94;
            let t97 = f64x8::splat(1.0) / t96;
            let t101 = param_c_x_2;
            let t102 = f64x8::splat(M_CBRT6);
            let t103 = t102 * t102;
            let t104 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t105 = (simd::cbrt(t104));
            let t106 = t105 * t105;
            let t107 = t103 * t106;
            let t108 = f64x8::splat(3.0) / f64x8::splat(10.0) * t107;
            let t109 = v_tau * t87;
            let t111 = f64x8::splat(1.0) / t89 / v_rho;
            let t112 = t109 * t111;
            let t113 = t108 - t112;
            let t114 = t101 * t113;
            let t115 = t108 + t112;
            let t116 = f64x8::splat(1.0) / t115;
            let t118 = param_c_x_0 + f64x8::splat(0.003840616724010807) * t86 * t92 * t97 + t114 * t116;
            let t119 = t83 * t118;
            let t123 = ((t5).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t14 * t21 * t119));
            let t124 = f64x8::splat(2.0) * t123;
            let t125 = ((t4).select(zeta_threshold, f64x8::splat(1.0)));
            let t126 = t9 * t11;
            let t129 = ((t4).select(f64x8::splat(1.0) / t16, f64x8::splat(1.0)));
            let t131 = t126 * t27 * t12 * t129;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t131;
            let t134 = ((t131).sqrt());
            let t137 = ((t131) * (t131).sqrt());
            let t139 = t6 * t6;
            let t140 = t139 * t24;
            let t141 = t140 * t10;
            let t142 = f64x8::splat(1.0) / t89;
            let t144 = t129 * t129;
            let t146 = t141 * t142 * t87 * t144;
            let t148 = f64x8::splat(3.79785) * t134 + f64x8::splat(0.8969) * t131 + f64x8::splat(0.204775) * t137 + f64x8::splat(0.123235) * t146;
            let t151 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t148;
            let t152 = (simd::ln(t151));
            let t154 = f64x8::splat(0.0621814) * t133 * t152;
            let t156 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t17, f64x8::splat(0.0)));
            let t160 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t12 - f64x8::splat(2.0));
            let t161 = (t19 + t156 - f64x8::splat(2.0)) * t160;
            let t163 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t131;
            let t168 = f64x8::splat(7.05945) * t134 + f64x8::splat(1.549425) * t131 + f64x8::splat(0.420775) * t137 + f64x8::splat(0.1562925) * t146;
            let t171 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t168;
            let t172 = (simd::ln(t171));
            let t176 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t131;
            let t181 = f64x8::splat(5.1785) * t134 + f64x8::splat(0.905775) * t131 + f64x8::splat(0.1100325) * t137 + f64x8::splat(0.1241775) * t146;
            let t184 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t181;
            let t185 = (simd::ln(t184));
            let t186 = t176 * t185;
            let t195 = ((t5).select(f64x8::splat(0.0), t125 * (-t154 + t161 * (-f64x8::splat(0.0310907) * t163 * t172 + t154 - f64x8::splat(0.0197516734986138) * t186) + f64x8::splat(0.0197516734986138) * t161 * t186) / f64x8::splat(2.0)));
            let t196 = param_c_ss_0;
            let t197 = t196 * v_sigma;
            let t199 = f64x8::splat(1.0) + f64x8::splat(0.46914023462026644) * t94;
            let t200 = f64x8::splat(1.0) / t199;
            let t204 = param_c_ss_1;
            let t205 = t204 * t113;
            let t207 = param_c_ss_2;
            let t208 = t113 * t113;
            let t209 = t207 * t208;
            let t210 = t115 * t115;
            let t211 = f64x8::splat(1.0) / t210;
            let t213 = param_c_ss_3;
            let t214 = v_sigma * v_sigma;
            let t215 = t214 * t214;
            let t216 = t215 * t214;
            let t217 = t213 * t216;
            let t218 = t88 * t88;
            let t219 = t218 * t218;
            let t220 = t219 * t219;
            let t221 = f64x8::splat(1.0) / t220;
            let t222 = t199 * t199;
            let t223 = t222 * t222;
            let t225 = f64x8::splat(1.0) / t223 / t222;
            let t226 = t221 * t225;
            let t229 = param_c_ss_4;
            let t230 = t208 * t208;
            let t231 = t229 * t230;
            let t232 = t210 * t210;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t231 * t233;
            let t239 = f64x8::splat(0.46914023462026644) * t197 * t92 * t200 + t205 * t116 + t209 * t211 + f64x8::splat(0.17058312527037534) * t217 * t226 + f64x8::splat(0.17058312527037534) * t234 * t216 * t221 * t225;
            let t241 = f64x8::splat(2.0) * t195 * t239;
            let t243 = t9 * t11 * t27;
            let t245 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t243;
            let t246 = ((t243).sqrt());
            let t249 = ((t243) * (t243).sqrt());
            let t252 = t140 * t10 * t142;
            let t254 = f64x8::splat(3.79785) * t246 + f64x8::splat(0.8969) * t243 + f64x8::splat(0.204775) * t249 + f64x8::splat(0.123235) * t252;
            let t257 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t254;
            let t258 = (simd::ln(t257));
            let t261 = ((t4).select(t17, f64x8::splat(1.0)));
            let t264 = (f64x8::splat(2.0) * t261 - f64x8::splat(2.0)) * t160;
            let t266 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t243;
            let t271 = f64x8::splat(5.1785) * t246 + f64x8::splat(0.905775) * t243 + f64x8::splat(0.1100325) * t249 + f64x8::splat(0.1241775) * t252;
            let t274 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t271;
            let t275 = (simd::ln(t274));
            let t280 = -f64x8::splat(0.0621814) * t245 * t258 + f64x8::splat(0.0197516734986138) * t264 * t266 * t275 - f64x8::splat(2.0) * t195;
            let t282 = param_c_os_1;
            let t284 = f64x8::splat(3.0) / f64x8::splat(5.0) * t107 * t112;
            let t285 = v_tau * v_tau;
            let t286 = t285 * t12;
            let t287 = t88 * v_rho;
            let t289 = f64x8::splat(1.0) / t20 / t287;
            let t291 = f64x8::splat(4.0) * t286 * t289;
            let t292 = t284 - t291;
            let t293 = t292 * t292;
            let t294 = t282 * t293;
            let t295 = t284 + t291;
            let t296 = t295 * t295;
            let t297 = f64x8::splat(1.0) / t296;
            let t299 = param_c_os_2;
            let t300 = t293 * t293;
            let t301 = t300 * t293;
            let t302 = t299 * t301;
            let t303 = t296 * t296;
            let t305 = f64x8::splat(1.0) / t303 / t296;
            let t307 = param_c_os_3;
            let t308 = t307 * t301;
            let t309 = (simd::cbrt(t94));
            let t310 = t305 * t309;
            let t312 = param_c_os_4;
            let t313 = t312 * t293;
            let t314 = t297 * t309;
            let t316 = t294 * t297 + t302 * t305 + t308 * t310 + t313 * t314 + param_c_os_0;
            let t317 = t280 * t316;
            let tzk0 = t124 + t241 + t317;
            acc_zk = tzk0;
            let t318 = t19 * t142;
            let t322 = t38 * t37;
            let t323 = f64x8::splat(1.0) / t322;
            let t325 = f64x8::splat(1.0) / t20 / v_rho;
            let t329 = t26 * t6 * t325 * t31 / f64x8::splat(54.0);
            let t330 = ((t36).select(-t329, f64x8::splat(0.0)));
            let t333 = t41 * t37;
            let t334 = f64x8::splat(1.0) / t333;
            let t337 = t41 * t322;
            let t338 = f64x8::splat(1.0) / t337;
            let t342 = f64x8::splat(1.0) / t47 / t37;
            let t346 = f64x8::splat(1.0) / t47 / t322;
            let t350 = f64x8::splat(1.0) / t47 / t333;
            let t354 = f64x8::splat(1.0) / t47 / t337;
            let t358 = f64x8::splat(1.0) / t59 / t37;
            let t362 = ((t36).select(f64x8::splat(0.0), -t329));
            let t364 = t72 * t70;
            let t368 = t69 * t63;
            let t369 = f64x8::splat(1.0) / t368;
            let t373 = t63 * t73;
            let t378 = t369 * t362 * t72 / f64x8::splat(2.0) - f64x8::splat(4.0) * t373 * t362 - t65 * t362 * t72;
            let t381 = -t364 * t362 + f64x8::splat(2.0) * t362 * t76 + f64x8::splat(2.0) * t63 * t378;
            let t385 = ((t35).select(-t323 * t330 / f64x8::splat(18.0) + t334 * t330 / f64x8::splat(240.0) - t338 * t330 / f64x8::splat(4480.0) + t342 * t330 / f64x8::splat(103680.0) - t346 * t330 / f64x8::splat(2838528.0) + t350 * t330 / f64x8::splat(89456640.0) - t354 * t330 / f64x8::splat(3185049600.0) + t358 * t330 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t362 * t79 - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t381));
            let t386 = t385 * t118;
            let t391 = f64x8::splat(1.0) / t89 / t287;
            let t392 = t87 * t391;
            let t396 = t85 * t214;
            let t397 = t218 * t88;
            let t399 = f64x8::splat(1.0) / t20 / t397;
            let t400 = t12 * t399;
            let t401 = t96 * t96;
            let t402 = f64x8::splat(1.0) / t401;
            let t406 = t101 * v_tau;
            let t407 = t92 * t116;
            let t410 = t114 * t211;
            let t411 = t109 * t91;
            let t414 = -f64x8::splat(0.010241644597362152) * t86 * t392 * t97 + f64x8::splat(7.866846304400802e-05) * t396 * t400 * t402 + f64x8::splat(5.0) / f64x8::splat(3.0) * t406 * t407 + f64x8::splat(5.0) / f64x8::splat(3.0) * t410 * t411;
            let t415 = t83 * t414;
            let t420 = ((t5).select(f64x8::splat(0.0), -t14 * t318 * t119 / f64x8::splat(64.0) - f64x8::splat(3.0) / f64x8::splat(64.0) * t14 * t21 * t386 - f64x8::splat(3.0) / f64x8::splat(64.0) * t14 * t21 * t415));
            let t422 = t325 * t12;
            let t423 = t129 * t152;
            let t426 = f64x8::splat(0.0011073470983333333) * t126 * t422 * t423;
            let t427 = t148 * t148;
            let t428 = f64x8::splat(1.0) / t427;
            let t429 = t133 * t428;
            let t432 = f64x8::splat(1.0) / t134 * t6 * t8;
            let t433 = t11 * t325;
            let t434 = t12 * t129;
            let t435 = t433 * t434;
            let t436 = t432 * t435;
            let t438 = t422 * t129;
            let t439 = t126 * t438;
            let t441 = ((t131).sqrt());
            let t443 = t441 * t6 * t8;
            let t444 = t443 * t435;
            let t446 = t111 * t87;
            let t448 = t141 * t446 * t144;
            let t450 = -f64x8::splat(0.632975) * t436 - f64x8::splat(0.29896666666666666) * t439 - f64x8::splat(0.1023875) * t444 - f64x8::splat(0.08215666666666667) * t448;
            let t451 = f64x8::splat(1.0) / t151;
            let t452 = t450 * t451;
            let t454 = f64x8::splat(1.0) * t429 * t452;
            let t455 = t129 * t172;
            let t459 = t168 * t168;
            let t460 = f64x8::splat(1.0) / t459;
            let t461 = t163 * t460;
            let t466 = -f64x8::splat(1.176575) * t436 - f64x8::splat(0.516475) * t439 - f64x8::splat(0.2103875) * t444 - f64x8::splat(0.104195) * t448;
            let t467 = f64x8::splat(1.0) / t171;
            let t468 = t466 * t467;
            let t471 = t129 * t185;
            let t475 = t181 * t181;
            let t476 = f64x8::splat(1.0) / t475;
            let t477 = t176 * t476;
            let t482 = -f64x8::splat(0.8630833333333333) * t436 - f64x8::splat(0.301925) * t439 - f64x8::splat(0.05501625) * t444 - f64x8::splat(0.082785) * t448;
            let t483 = f64x8::splat(1.0) / t184;
            let t484 = t482 * t483;
            let t489 = t161 * t9;
            let t490 = t434 * t185;
            let t494 = t161 * t176;
            let t496 = t476 * t482 * t483;
            let t502 = ((t5).select(f64x8::splat(0.0), t125 * (t426 + t454 + t161 * (f64x8::splat(0.0005323764196666666) * t126 * t422 * t455 + f64x8::splat(1.0) * t461 * t468 - t426 - t454 + f64x8::splat(0.00018311447306006544) * t126 * t422 * t471 + f64x8::splat(0.5848223622634646) * t477 * t484) - f64x8::splat(0.00018311447306006544) * t489 * t433 * t490 - f64x8::splat(0.5848223622634646) * t494 * t496) / f64x8::splat(2.0)));
            let t503 = t502 * t239;
            let t508 = t196 * t214;
            let t509 = f64x8::splat(1.0) / t222;
            let t513 = t204 * v_tau;
            let t516 = t205 * t211;
            let t519 = t207 * t113;
            let t520 = t519 * t211;
            let t523 = t210 * t115;
            let t524 = f64x8::splat(1.0) / t523;
            let t525 = t209 * t524;
            let t528 = t220 * v_rho;
            let t529 = f64x8::splat(1.0) / t528;
            let t530 = t529 * t225;
            let t533 = t214 * v_sigma;
            let t534 = t215 * t533;
            let t535 = t213 * t534;
            let t536 = t220 * t287;
            let t538 = f64x8::splat(1.0) / t89 / t536;
            let t539 = t222 * t199;
            let t541 = f64x8::splat(1.0) / t223 / t539;
            let t543 = t538 * t541 * t87;
            let t547 = t229 * t208 * t113;
            let t548 = t233 * t216;
            let t549 = t547 * t548;
            let t550 = t220 * t88;
            let t552 = f64x8::splat(1.0) / t89 / t550;
            let t554 = t552 * t225 * t109;
            let t558 = f64x8::splat(1.0) / t232 / t115;
            let t559 = t558 * t216;
            let t560 = t231 * t559;
            let t568 = t541 * t87;
            let t572 = -f64x8::splat(1.2510406256540438) * t197 * t392 * t200 + f64x8::splat(1.1738269852776462) * t508 * t400 * t509 + f64x8::splat(5.0) / f64x8::splat(3.0) * t513 * t407 + f64x8::splat(5.0) / f64x8::splat(3.0) * t516 * t411 + f64x8::splat(10.0) / f64x8::splat(3.0) * t520 * t411 + f64x8::splat(10.0) / f64x8::splat(3.0) * t525 * t411 - f64x8::splat(2.7293300043260054) * t217 * t530 + f64x8::splat(1.2804385185856348) * t535 * t543 + f64x8::splat(1.1372208351358355) * t549 * t554 + f64x8::splat(1.1372208351358355) * t560 * t554 - f64x8::splat(2.7293300043260054) * t234 * t216 * t529 * t225 + f64x8::splat(1.2804385185856348) * t234 * t534 * t538 * t568;
            let t573 = t195 * t572;
            let t578 = t254 * t254;
            let t579 = f64x8::splat(1.0) / t578;
            let t580 = t245 * t579;
            let t582 = f64x8::splat(1.0) / t246 * t6;
            let t583 = t8 * t11;
            let t584 = t583 * t325;
            let t585 = t582 * t584;
            let t587 = t9 * t433;
            let t589 = ((t243).sqrt());
            let t590 = t589 * t6;
            let t591 = t590 * t584;
            let t594 = t140 * t10 * t111;
            let t596 = -f64x8::splat(0.632975) * t585 - f64x8::splat(0.29896666666666666) * t587 - f64x8::splat(0.1023875) * t591 - f64x8::splat(0.08215666666666667) * t594;
            let t597 = f64x8::splat(1.0) / t257;
            let t598 = t596 * t597;
            let t601 = t264 * t6;
            let t606 = t264 * t266;
            let t607 = t271 * t271;
            let t608 = f64x8::splat(1.0) / t607;
            let t613 = -f64x8::splat(0.8630833333333333) * t585 - f64x8::splat(0.301925) * t587 - f64x8::splat(0.05501625) * t591 - f64x8::splat(0.082785) * t594;
            let t615 = f64x8::splat(1.0) / t274;
            let t616 = t608 * t613 * t615;
            let t620 = f64x8::splat(0.0011073470983333333) * t9 * t433 * t258 + f64x8::splat(1.0) * t580 * t598 - f64x8::splat(0.00018311447306006544) * t601 * t583 * t325 * t275 - f64x8::splat(0.5848223622634646) * t606 * t616 - f64x8::splat(2.0) * t502;
            let t621 = t620 * t316;
            let t622 = t282 * t292;
            let t623 = t107 * t411;
            let t625 = f64x8::splat(1.0) / t20 / t218;
            let t627 = f64x8::splat(40.0) / f64x8::splat(3.0) * t286 * t625;
            let t628 = -t623 + t627;
            let t632 = t296 * t295;
            let t633 = f64x8::splat(1.0) / t632;
            let t634 = -t623 - t627;
            let t635 = t633 * t634;
            let t638 = t300 * t292;
            let t639 = t299 * t638;
            let t640 = t305 * t628;
            let t644 = f64x8::splat(1.0) / t303 / t632;
            let t648 = t307 * t638;
            let t652 = t644 * t309;
            let t656 = t308 * t305;
            let t657 = t309 * t309;
            let t658 = f64x8::splat(1.0) / t657;
            let t659 = t658 * v_sigma;
            let t660 = t659 * t392;
            let t663 = t312 * t292;
            let t667 = t633 * t309;
            let t668 = t667 * t634;
            let t671 = t313 * t297;
            let t674 = f64x8::splat(2.0) * t622 * t297 * t628 - f64x8::splat(2.0) * t294 * t635 + f64x8::splat(6.0) * t639 * t640 - f64x8::splat(6.0) * t302 * t644 * t634 + f64x8::splat(6.0) * t648 * t310 * t628 - f64x8::splat(6.0) * t308 * t652 * t634 - f64x8::splat(8.0) / f64x8::splat(9.0) * t656 * t660 + f64x8::splat(2.0) * t663 * t314 * t628 - f64x8::splat(2.0) * t313 * t668 - f64x8::splat(8.0) / f64x8::splat(9.0) * t671 * t660;
            let t675 = t280 * t674;
            let tvrho0 = t124 + t241 + t317 + v_rho * (f64x8::splat(2.0) * t420 + f64x8::splat(2.0) * t503 + f64x8::splat(2.0) * t573 + t621 + t675);
            acc_vrho = tvrho0;
            let t678 = t85 * t87;
            let t682 = t218 * v_rho;
            let t684 = f64x8::splat(1.0) / t20 / t682;
            let t685 = t12 * t684;
            let t689 = f64x8::splat(0.003840616724010807) * t678 * t91 * t97 - f64x8::splat(2.9500673641503008e-05) * t86 * t685 * t402;
            let t690 = t83 * t689;
            let t694 = ((t5).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t14 * t21 * t690));
            let t695 = f64x8::splat(2.0) * t694;
            let t696 = t196 * t87;
            let t703 = t215 * v_sigma;
            let t704 = t213 * t703;
            let t708 = t552 * t541 * t87;
            let t715 = t216 * t552;
            let t719 = f64x8::splat(0.46914023462026644) * t696 * t91 * t200 - f64x8::splat(0.44018511947911726) * t197 * t685 * t509 + f64x8::splat(1.023498751622252) * t704 * t226 - f64x8::splat(0.4801644444696131) * t217 * t708 + f64x8::splat(1.023498751622252) * t234 * t703 * t221 * t225 - f64x8::splat(0.4801644444696131) * t234 * t715 * t568;
            let t721 = f64x8::splat(2.0) * t195 * t719;
            let t722 = t658 * t87;
            let t723 = t722 * t91;
            let t727 = t656 * t723 / f64x8::splat(3.0) + t671 * t723 / f64x8::splat(3.0);
            let t728 = t280 * t727;
            let tvsigma0 = v_rho * (t695 + t721 + t728);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t730 = t101 * t87;
            let t731 = t111 * t116;
            let t733 = t211 * t87;
            let t734 = t733 * t111;
            let t736 = -t114 * t734 - t730 * t731;
            let t737 = t83 * t736;
            let t741 = ((t5).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t14 * t21 * t737));
            let t742 = f64x8::splat(2.0) * t741;
            let t743 = t204 * t87;
            let t748 = t524 * t87;
            let t752 = t547 * t233;
            let t754 = f64x8::splat(1.0) / t89 / t528;
            let t756 = t225 * t87;
            let t757 = t216 * t754 * t756;
            let t760 = t231 * t558;
            let t763 = -t743 * t731 - t205 * t734 - f64x8::splat(2.0) * t519 * t734 - f64x8::splat(2.0) * t209 * t748 * t111 - f64x8::splat(0.6823325010815013) * t752 * t757 - f64x8::splat(0.6823325010815013) * t760 * t757;
            let t765 = f64x8::splat(2.0) * t195 * t763;
            let t767 = f64x8::splat(3.0) / f64x8::splat(5.0) * t107 * t446;
            let t768 = v_tau * t12;
            let t770 = f64x8::splat(8.0) * t768 * t289;
            let t771 = t767 - t770;
            let t772 = t297 * t771;
            let t775 = t767 + t770;
            let t776 = t633 * t775;
            let t779 = t305 * t771;
            let t782 = t644 * t775;
            let t791 = t314 * t771;
            let t794 = t667 * t775;
            let t797 = -f64x8::splat(6.0) * t308 * t652 * t775 + f64x8::splat(6.0) * t648 * t310 * t771 - f64x8::splat(2.0) * t294 * t776 - f64x8::splat(6.0) * t302 * t782 - f64x8::splat(2.0) * t313 * t794 + f64x8::splat(2.0) * t622 * t772 + f64x8::splat(6.0) * t639 * t779 + f64x8::splat(2.0) * t663 * t791;
            let t798 = t280 * t797;
            let tvtau0 = v_rho * (t742 + t765 + t798);
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
