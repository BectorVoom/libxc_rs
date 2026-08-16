//! GGA_X_HJS_B88_V2 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs_b88_v2.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hjs_b88_v2_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t17 = t16 * t7;
        let t18 = piecewise5(t10, t11, t14, t15, t17);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = t2 * t2;
        let t29 = param_hyb_omega_0 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t35 = 1.0 + t17 <= zeta_threshold;
        let t37 = 1.0 - t17 <= zeta_threshold;
        let t38 = piecewise5(t35, t11, t37, t15, t17);
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3(t39);
        let t42 = piecewise3(t40, t21, t41);
        let t43 = 1.0 / t42;
        let t44 = 1.0 / t26;
        let t45 = t43 * t44;
        let t46 = M_CBRT6;
        let t47 = t46 * t46;
        let t48 = t47 * t32;
        let t49 = f64::sqrt(sigma0);
        let t50 = pow_1_3(rho0);
        let t52 = 1.0 / t50 / rho0;
        let t56 = f64::exp(-t48 * t49 * t52 / 12.0);
        let t57 = f64::exp(20.0);
        let t59 = 1.0 / (t57 - 1.0);
        let t60 = t56 + t59;
        let t62 = 1.0 / (1.0 + t59);
        let t64 = f64::ln(t60 * t62);
        let t65 = t64 * t64;
        let t66 = param_a_0;
        let t68 = param_a_1;
        let t69 = t65 * t64;
        let t71 = param_a_2;
        let t72 = t65 * t65;
        let t74 = param_a_3;
        let t75 = t72 * t64;
        let t77 = param_a_4;
        let t78 = t72 * t65;
        let t80 = param_a_5;
        let t81 = t72 * t69;
        let t83 = t66 * t65 - t68 * t69 + t71 * t72 - t74 * t75 + t77 * t78 - t80 * t81;
        let t84 = t65 * t83;
        let t85 = param_b_0;
        let t87 = param_b_1;
        let t89 = param_b_2;
        let t91 = param_b_3;
        let t93 = param_b_4;
        let t95 = param_b_5;
        let t97 = param_b_6;
        let t99 = param_b_7;
        let t100 = t72 * t72;
        let t102 = param_b_8;
        let t105 = -t102 * t100 * t64 + t99 * t100 - t85 * t64 + t87 * t65 - t89 * t69 + t91 * t72 - t93 * t75 + t95 * t78 - t97 * t81 + 1.0;
        let t106 = 1.0 / t105;
        let t107 = t84 * t106;
        let t108 = 0.1e-9 < t107;
        let t109 = piecewise3(t108, t107, 0.1e-9);
        let t110 = param_hyb_omega_0 * param_hyb_omega_0;
        let t111 = t110 * t2;
        let t112 = t31 * t31;
        let t113 = 1.0 / t112;
        let t114 = t42 * t42;
        let t115 = 1.0 / t114;
        let t116 = t113 * t115;
        let t117 = t26 * t26;
        let t118 = 1.0 / t117;
        let t120 = t111 * t116 * t118;
        let t122 = 0.60965e0 + t109 + t120 / 3.0;
        let t123 = f64::sqrt(t122);
        let t124 = 1.0 / t123;
        let t126 = t33 * t45 * t124;
        let t128 = 1.0 - t126 / 3.0;
        let t129 = 0.60965e0 + t109;
        let t130 = 1.0 / t129;
        let t134 = 1.0 + t65 / 4.0;
        let t135 = 1.0 / t134;
        let t139 = 1.0 + 0.31215633538451261314e0 * t65 * t135 + 0.42141105276909202774e1 * t109;
        let t141 = 1.0 / t30;
        let t142 = t110 * param_hyb_omega_0 * t141;
        let t143 = t114 * t42;
        let t144 = 1.0 / t143;
        let t145 = t144 * t7;
        let t147 = 1.0 / t123 / t122;
        let t149 = t142 * t145 * t147;
        let t151 = 2.0 - t126 + t149 / 3.0;
        let t152 = t139 * t151;
        let t153 = t129 * t129;
        let t154 = 1.0 / t153;
        let t160 = t153 * t129;
        let t162 = f64::sqrt(t129);
        let t163 = t162 * t160;
        let t164 = f64::sqrt(M_PI);
        let t165 = 4.0 / 5.0 * t164;
        let t166 = f64::sqrt(t109);
        let t169 = 0.0 < 0.7572109999e0 + t109;
        let t171 = piecewise3(t169, 0.757211e0 + t109, 0.1e-9);
        let t172 = f64::sqrt(t171);
        let t174 = t165 + 12.0 / 5.0 * t166 - 12.0 / 5.0 * t172;
        let t176 = 0.474596e-1 * t139 * t129 + 0.28363733333333333333e-1 * t153 - 0.9086532e0 * t160 - t163 * t174;
        let t179 = t110 * t110;
        let t181 = t179 * param_hyb_omega_0 * t2;
        let t183 = 1.0 / t112 / t30;
        let t184 = t181 * t183;
        let t185 = t114 * t114;
        let t187 = 1.0 / t185 / t42;
        let t189 = 1.0 / t117 / t6;
        let t190 = t187 * t189;
        let t191 = t122 * t122;
        let t193 = 1.0 / t123 / t191;
        let t197 = 8.0 - 5.0 * t126 + 10.0 / 3.0 * t149 - t184 * t190 * t193 / 3.0;
        let t198 = t176 * t197;
        let t199 = 1.0 / t160;
        let t203 = 3.0 * t120;
        let t204 = 9.0 * t109 + t203;
        let t205 = f64::sqrt(t204);
        let t207 = 9.0 * t171 + t203;
        let t208 = f64::sqrt(t207);
        let t210 = t205 / 3.0 - t208 / 3.0;
        let t214 = t32 * t43;
        let t216 = t29 * t214 * t44;
        let t218 = t216 / 3.0 + t205 / 3.0;
        let t220 = t216 / 3.0 + t123;
        let t221 = 1.0 / t220;
        let t223 = f64::ln(t218 * t221);
        let t227 = t216 / 3.0 + t208 / 3.0;
        let t229 = f64::ln(t227 * t221);
        let t232 = 0.757211e0 + 0.47272888888888888889e-1 * t128 * t130 + 0.26366444444444444444e-1 * t152 * t154 - t198 * t199 / 9.0 + 2.0 / 3.0 * t33 * t45 * t210 + 2.0 * t109 * t223 - 2.0 * t171 * t229;
        let t236 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t232);
        let t237 = rho1 <= dens_threshold;
        let t238 = -t16;
        let t240 = piecewise5(t14, t11, t10, t15, t238 * t7);
        let t241 = 1.0 + t240;
        let t242 = t241 <= zeta_threshold;
        let t243 = pow_1_3(t241);
        let t245 = piecewise3(t242, t22, t243 * t241);
        let t246 = t245 * t26;
        let t247 = piecewise5(t37, t11, t35, t15, -t17);
        let t248 = 1.0 + t247;
        let t249 = t248 <= zeta_threshold;
        let t250 = pow_1_3(t248);
        let t251 = piecewise3(t249, t21, t250);
        let t252 = 1.0 / t251;
        let t253 = t252 * t44;
        let t254 = f64::sqrt(sigma2);
        let t255 = pow_1_3(rho1);
        let t257 = 1.0 / t255 / rho1;
        let t261 = f64::exp(-t48 * t254 * t257 / 12.0);
        let t262 = t261 + t59;
        let t264 = f64::ln(t262 * t62);
        let t265 = t264 * t264;
        let t267 = t265 * t264;
        let t269 = t265 * t265;
        let t271 = t269 * t264;
        let t273 = t269 * t265;
        let t275 = t269 * t267;
        let t277 = t66 * t265 - t68 * t267 + t71 * t269 - t74 * t271 + t77 * t273 - t80 * t275;
        let t278 = t265 * t277;
        let t286 = t269 * t269;
        let t290 = -t102 * t286 * t264 - t85 * t264 + t87 * t265 - t89 * t267 + t91 * t269 - t93 * t271 + t95 * t273 - t97 * t275 + t99 * t286 + 1.0;
        let t291 = 1.0 / t290;
        let t292 = t278 * t291;
        let t293 = 0.1e-9 < t292;
        let t294 = piecewise3(t293, t292, 0.1e-9);
        let t295 = t251 * t251;
        let t296 = 1.0 / t295;
        let t297 = t113 * t296;
        let t299 = t111 * t297 * t118;
        let t301 = 0.60965e0 + t294 + t299 / 3.0;
        let t302 = f64::sqrt(t301);
        let t303 = 1.0 / t302;
        let t305 = t33 * t253 * t303;
        let t307 = 1.0 - t305 / 3.0;
        let t308 = 0.60965e0 + t294;
        let t309 = 1.0 / t308;
        let t313 = 1.0 + t265 / 4.0;
        let t314 = 1.0 / t313;
        let t318 = 1.0 + 0.31215633538451261314e0 * t265 * t314 + 0.42141105276909202774e1 * t294;
        let t319 = t295 * t251;
        let t320 = 1.0 / t319;
        let t321 = t320 * t7;
        let t323 = 1.0 / t302 / t301;
        let t325 = t142 * t321 * t323;
        let t327 = 2.0 - t305 + t325 / 3.0;
        let t328 = t318 * t327;
        let t329 = t308 * t308;
        let t330 = 1.0 / t329;
        let t336 = t329 * t308;
        let t338 = f64::sqrt(t308);
        let t339 = t338 * t336;
        let t340 = f64::sqrt(t294);
        let t343 = 0.0 < 0.7572109999e0 + t294;
        let t345 = piecewise3(t343, 0.757211e0 + t294, 0.1e-9);
        let t346 = f64::sqrt(t345);
        let t348 = t165 + 12.0 / 5.0 * t340 - 12.0 / 5.0 * t346;
        let t350 = 0.474596e-1 * t318 * t308 + 0.28363733333333333333e-1 * t329 - 0.9086532e0 * t336 - t339 * t348;
        let t353 = t295 * t295;
        let t355 = 1.0 / t353 / t251;
        let t356 = t355 * t189;
        let t357 = t301 * t301;
        let t359 = 1.0 / t302 / t357;
        let t363 = 8.0 - 5.0 * t305 + 10.0 / 3.0 * t325 - t184 * t356 * t359 / 3.0;
        let t364 = t350 * t363;
        let t365 = 1.0 / t336;
        let t369 = 3.0 * t299;
        let t370 = 9.0 * t294 + t369;
        let t371 = f64::sqrt(t370);
        let t373 = 9.0 * t345 + t369;
        let t374 = f64::sqrt(t373);
        let t376 = t371 / 3.0 - t374 / 3.0;
        let t380 = t32 * t252;
        let t382 = t29 * t380 * t44;
        let t384 = t382 / 3.0 + t371 / 3.0;
        let t386 = t382 / 3.0 + t302;
        let t387 = 1.0 / t386;
        let t389 = f64::ln(t384 * t387);
        let t393 = t382 / 3.0 + t374 / 3.0;
        let t395 = f64::ln(t393 * t387);
        let t398 = 0.757211e0 + 0.47272888888888888889e-1 * t307 * t309 + 0.26366444444444444444e-1 * t328 * t330 - t364 * t365 / 9.0 + 2.0 / 3.0 * t33 * t253 * t376 + 2.0 * t294 * t389 - 2.0 * t345 * t395;
        let t402 = piecewise3(t237, 0.0, -3.0 / 8.0 * t5 * t246 * t398);
        let tzk0 = t236 + t402;
        zk[ip] += tzk0;
    }
}
