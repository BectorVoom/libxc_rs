//! MGGA_X_MN12 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mn12.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mn12_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_6: f64,
    param_c_12: f64,
    param_c_13: f64,
    param_c_14: f64,
    param_c_11: f64,
    param_c_16: f64,
    param_c_17: f64,
    param_c_15: f64,
    param_c_19: f64,
    param_c_20: f64,
    param_c_21: f64,
    param_c_22: f64,
    param_c_18: f64,
    param_c_24: f64,
    param_c_25: f64,
    param_c_26: f64,
    param_c_23: f64,
    param_c_28: f64,
    param_c_29: f64,
    param_c_27: f64,
    param_c_31: f64,
    param_c_32: f64,
    param_c_33: f64,
    param_c_30: f64,
    param_c_35: f64,
    param_c_36: f64,
    param_c_34: f64,
    param_c_38: f64,
    param_c_39: f64,
    param_c_37: f64,
    param_c_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t18 = t17 * t8;
        let t19 = piecewise5(t11, t12, t15, t16, t18);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = param_c_0;
        let t30 = param_c_1;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = 3.0 / 10.0 * t32 * t35;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / rho0;
        let t42 = tau0 * t41;
        let t43 = t37 - t42;
        let t44 = t30 * t43;
        let t45 = t37 + t42;
        let t46 = 1.0 / t45;
        let t48 = param_c_2;
        let t49 = t43 * t43;
        let t50 = t48 * t49;
        let t51 = t45 * t45;
        let t52 = 1.0 / t51;
        let t54 = param_c_3;
        let t55 = t49 * t43;
        let t56 = t54 * t55;
        let t57 = t51 * t45;
        let t58 = 1.0 / t57;
        let t60 = param_c_4;
        let t61 = t49 * t49;
        let t62 = t60 * t61;
        let t63 = t51 * t51;
        let t64 = 1.0 / t63;
        let t66 = param_c_5;
        let t68 = t66 * t61 * t43;
        let t70 = 1.0 / t63 / t45;
        let t72 = param_c_6;
        let t73 = param_c_7;
        let t74 = t73 * t43;
        let t76 = param_c_8;
        let t77 = t76 * t49;
        let t79 = param_c_9;
        let t80 = t79 * t55;
        let t82 = param_c_10;
        let t83 = t82 * t61;
        let t85 = t74 * t46 + t77 * t52 + t80 * t58 + t83 * t64 + t72;
        let t86 = t85 * sigma0;
        let t87 = rho0 * rho0;
        let t89 = 1.0 / t39 / t87;
        let t92 = 1.0 + 0.004 * sigma0 * t89;
        let t93 = 1.0 / t92;
        let t94 = t89 * t93;
        let t97 = param_c_11;
        let t98 = param_c_12;
        let t99 = t98 * t43;
        let t101 = param_c_13;
        let t102 = t101 * t49;
        let t104 = param_c_14;
        let t105 = t104 * t55;
        let t107 = t102 * t52 + t105 * t58 + t99 * t46 + t97;
        let t108 = sigma0 * sigma0;
        let t109 = t107 * t108;
        let t110 = t87 * t87;
        let t111 = t110 * rho0;
        let t113 = 1.0 / t38 / t111;
        let t114 = t92 * t92;
        let t115 = 1.0 / t114;
        let t116 = t113 * t115;
        let t119 = param_c_15;
        let t120 = param_c_16;
        let t121 = t120 * t43;
        let t123 = param_c_17;
        let t124 = t123 * t49;
        let t126 = t121 * t46 + t124 * t52 + t119;
        let t127 = t108 * sigma0;
        let t128 = t126 * t127;
        let t129 = t110 * t110;
        let t130 = 1.0 / t129;
        let t131 = t114 * t92;
        let t132 = 1.0 / t131;
        let t133 = t130 * t132;
        let t136 = param_c_18;
        let t137 = param_c_19;
        let t138 = t137 * t43;
        let t140 = param_c_20;
        let t141 = t140 * t49;
        let t143 = param_c_21;
        let t144 = t143 * t55;
        let t146 = param_c_22;
        let t147 = t146 * t61;
        let t149 = t138 * t46 + t141 * t52 + t144 * t58 + t147 * t64 + t136;
        let t151 = M_CBRT2;
        let t152 = 1.0 / t27 * t151;
        let t154 = 1.0 + t18 <= zeta_threshold;
        let t156 = 1.0 - t18 <= zeta_threshold;
        let t157 = piecewise5(t154, t12, t156, t16, t18);
        let t158 = 1.0 + t157;
        let t159 = 1.0 / t158;
        let t160 = pow_1_3(t159);
        let t163 = 1.0 + 0.4 * t152 * t160;
        let t164 = 1.0 / t163;
        let t166 = param_c_23;
        let t167 = param_c_24;
        let t168 = t167 * t43;
        let t170 = param_c_25;
        let t171 = t170 * t49;
        let t173 = param_c_26;
        let t174 = t173 * t55;
        let t176 = t168 * t46 + t171 * t52 + t174 * t58 + t166;
        let t177 = t176 * sigma0;
        let t178 = t94 * t164;
        let t181 = param_c_27;
        let t182 = param_c_28;
        let t183 = t182 * t43;
        let t185 = param_c_29;
        let t186 = t185 * t49;
        let t188 = t183 * t46 + t186 * t52 + t181;
        let t189 = t188 * t108;
        let t190 = t116 * t164;
        let t193 = param_c_30;
        let t194 = param_c_31;
        let t195 = t194 * t43;
        let t197 = param_c_32;
        let t198 = t197 * t49;
        let t200 = param_c_33;
        let t201 = t200 * t55;
        let t203 = t195 * t46 + t198 * t52 + t201 * t58 + t193;
        let t204 = t163 * t163;
        let t205 = 1.0 / t204;
        let t207 = param_c_34;
        let t208 = param_c_35;
        let t209 = t208 * t43;
        let t211 = param_c_36;
        let t212 = t211 * t49;
        let t214 = t209 * t46 + t212 * t52 + t207;
        let t215 = t214 * sigma0;
        let t216 = t94 * t205;
        let t219 = param_c_37;
        let t220 = param_c_38;
        let t221 = t220 * t43;
        let t223 = param_c_39;
        let t224 = t223 * t49;
        let t226 = t221 * t46 + t224 * t52 + t219;
        let t227 = t204 * t163;
        let t228 = 1.0 / t227;
        let t230 = t29 + t44 * t46 + t50 * t52 + t56 * t58 + t62 * t64 + t68 * t70 + 0.004 * t86 * t94 + 1.6e-05 * t109 * t116 + 6.4e-08 * t128 * t133 + t149 * t164 + 0.004 * t177 * t178 + 1.6e-05 * t189 * t190 + t203 * t205 + 0.004 * t215 * t216 + t226 * t228;
        let t234 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t230);
        let t235 = rho1 <= dens_threshold;
        let t236 = -t17;
        let t238 = piecewise5(t15, t12, t11, t16, t236 * t8);
        let t239 = 1.0 + t238;
        let t240 = t239 <= zeta_threshold;
        let t241 = pow_1_3(t239);
        let t243 = piecewise3(t240, t23, t241 * t239);
        let t244 = t243 * t27;
        let t245 = pow_1_3(rho1);
        let t246 = t245 * t245;
        let t248 = 1.0 / t246 / rho1;
        let t249 = tau1 * t248;
        let t250 = t37 - t249;
        let t251 = t30 * t250;
        let t252 = t37 + t249;
        let t253 = 1.0 / t252;
        let t255 = t250 * t250;
        let t256 = t48 * t255;
        let t257 = t252 * t252;
        let t258 = 1.0 / t257;
        let t260 = t255 * t250;
        let t261 = t54 * t260;
        let t262 = t257 * t252;
        let t263 = 1.0 / t262;
        let t265 = t255 * t255;
        let t266 = t60 * t265;
        let t267 = t257 * t257;
        let t268 = 1.0 / t267;
        let t271 = t66 * t265 * t250;
        let t273 = 1.0 / t267 / t252;
        let t275 = t73 * t250;
        let t277 = t76 * t255;
        let t279 = t79 * t260;
        let t281 = t82 * t265;
        let t283 = t275 * t253 + t277 * t258 + t279 * t263 + t281 * t268 + t72;
        let t284 = t283 * sigma2;
        let t285 = rho1 * rho1;
        let t287 = 1.0 / t246 / t285;
        let t290 = 1.0 + 0.004 * sigma2 * t287;
        let t291 = 1.0 / t290;
        let t292 = t287 * t291;
        let t295 = t98 * t250;
        let t297 = t101 * t255;
        let t299 = t104 * t260;
        let t301 = t295 * t253 + t297 * t258 + t299 * t263 + t97;
        let t302 = sigma2 * sigma2;
        let t303 = t301 * t302;
        let t304 = t285 * t285;
        let t305 = t304 * rho1;
        let t307 = 1.0 / t245 / t305;
        let t308 = t290 * t290;
        let t309 = 1.0 / t308;
        let t310 = t307 * t309;
        let t313 = t120 * t250;
        let t315 = t123 * t255;
        let t317 = t313 * t253 + t315 * t258 + t119;
        let t318 = t302 * sigma2;
        let t319 = t317 * t318;
        let t320 = t304 * t304;
        let t321 = 1.0 / t320;
        let t322 = t308 * t290;
        let t323 = 1.0 / t322;
        let t324 = t321 * t323;
        let t327 = t137 * t250;
        let t329 = t140 * t255;
        let t331 = t143 * t260;
        let t333 = t146 * t265;
        let t335 = t327 * t253 + t329 * t258 + t331 * t263 + t333 * t268 + t136;
        let t336 = piecewise5(t156, t12, t154, t16, -t18);
        let t337 = 1.0 + t336;
        let t338 = 1.0 / t337;
        let t339 = pow_1_3(t338);
        let t342 = 1.0 + 0.4 * t152 * t339;
        let t343 = 1.0 / t342;
        let t345 = t167 * t250;
        let t347 = t170 * t255;
        let t349 = t173 * t260;
        let t351 = t345 * t253 + t347 * t258 + t349 * t263 + t166;
        let t352 = t351 * sigma2;
        let t353 = t292 * t343;
        let t356 = t182 * t250;
        let t358 = t185 * t255;
        let t360 = t356 * t253 + t358 * t258 + t181;
        let t361 = t360 * t302;
        let t362 = t310 * t343;
        let t365 = t194 * t250;
        let t367 = t197 * t255;
        let t369 = t200 * t260;
        let t371 = t365 * t253 + t367 * t258 + t369 * t263 + t193;
        let t372 = t342 * t342;
        let t373 = 1.0 / t372;
        let t375 = t208 * t250;
        let t377 = t211 * t255;
        let t379 = t375 * t253 + t377 * t258 + t207;
        let t380 = t379 * sigma2;
        let t381 = t292 * t373;
        let t384 = t220 * t250;
        let t386 = t223 * t255;
        let t388 = t384 * t253 + t386 * t258 + t219;
        let t389 = t372 * t342;
        let t390 = 1.0 / t389;
        let t392 = t29 + t251 * t253 + t256 * t258 + t261 * t263 + t266 * t268 + t271 * t273 + 0.004 * t284 * t292 + 1.6e-05 * t303 * t310 + 6.4e-08 * t319 * t324 + t335 * t343 + 0.004 * t352 * t353 + 1.6e-05 * t361 * t362 + t371 * t373 + 0.004 * t380 * t381 + t388 * t390;
        let t396 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t244 * t392);
        let tzk0 = t234 + t396;
        zk[ip] += tzk0;
    }
}
