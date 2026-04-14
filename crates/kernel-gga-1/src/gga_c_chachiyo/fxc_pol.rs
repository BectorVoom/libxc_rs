//! GGA_C_CHACHIYO fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 52 shared lines across all orders.
//! Delta: 136 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_chachiyo_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (52 lines) ---
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = f64::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = f64::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3(zeta_threshold);
        let t42 = t41 * t41;
        let t43 = pow_1_3(t39);
        let t44 = t43 * t43;
        let t45 = piecewise3(t40, t42, t44);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3(t46);
        let t49 = t48 * t48;
        let t50 = piecewise3(t47, t42, t49);
        let t52 = t45 / 2.0 + t50 / 2.0;
        let t53 = t52 * t52;
        let t56 = -2.0 * t53 * t52 + 2.0;
        let t58 = t35 * t56 + t25;
        let t59 = M_CBRTPI;
        let t60 = t2 * t59;
        let t61 = t9 * t9;
        let t63 = 1.0 / t10 / t61;
        let t65 = sigma0 + 2.0 * sigma1 + sigma2;
        let t69 = 1.0 + t60 * t63 * t65 / 48.0;
        let t70 = 1.0 / t58;
        let t71 = param_h * t70;
        let t72 = f64::powf(t69, t71);
        let tzk0 = t58 * t72;
        zk[ip] += tzk0;
        // --- vxc delta (56 lines) ---
        let t74 = t8 / t19;
        let t78 = t18 / t10;
        let t81 = t3 * t74 / 9.0 + 2.0 / 9.0 * t14 * t78;
        let t83 = 1.0 / t23;
        let t84 = param_ap * t81 * t83;
        let t89 = t26 * t74 / 9.0 + 2.0 / 9.0 * t29 * t78;
        let t91 = 1.0 / t32;
        let t93 = param_af * t89 * t91 - t84;
        let t94 = t93 * t56;
        let t95 = t35 * t53;
        let t96 = 1.0 / t43;
        let t97 = 1.0 / t61;
        let t98 = t36 * t97;
        let t99 = t37 - t98;
        let t102 = piecewise3(t40, 0.0, 2.0 / 3.0 * t96 * t99);
        let t103 = 1.0 / t48;
        let t104 = -t99;
        let t107 = piecewise3(t47, 0.0, 2.0 / 3.0 * t103 * t104);
        let t109 = t102 / 2.0 + t107 / 2.0;
        let t112 = -6.0 * t95 * t109 + t84 + t94;
        let t113 = t9 * t112;
        let t115 = t9 * t58;
        let t116 = t58 * t58;
        let t117 = 1.0 / t116;
        let t118 = param_h * t117;
        let t119 = f64::ln(t69);
        let t120 = t112 * t119;
        let t122 = t71 * t2;
        let t123 = t61 * t9;
        let t125 = 1.0 / t10 / t123;
        let t126 = t59 * t125;
        let t127 = 1.0 / t69;
        let t128 = t65 * t127;
        let t129 = t126 * t128;
        let t131 = 7.0 / 144.0 * t122 * t129;
        let t132 = -t118 * t120 - t131;
        let t133 = t72 * t132;
        let tvrho0 = t113 * t72 + t115 * t133 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t135 = -t37 - t98;
        let t138 = piecewise3(t40, 0.0, 2.0 / 3.0 * t96 * t135);
        let t139 = -t135;
        let t142 = piecewise3(t47, 0.0, 2.0 / 3.0 * t103 * t139);
        let t144 = t138 / 2.0 + t142 / 2.0;
        let t147 = -6.0 * t95 * t144 + t84 + t94;
        let t148 = t9 * t147;
        let t150 = t147 * t119;
        let t152 = -t118 * t150 - t131;
        let t153 = t72 * t152;
        let tvrho1 = t115 * t153 + t148 * t72 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t156 = 1.0 / t10 / t9;
        let t157 = t156 * t72;
        let t159 = t60 * t127;
        let t160 = t157 * param_h * t159;
        let tvsigma0 = t160 / 48.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = t160 / 24.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        // --- fxc delta (this level) (136 lines) ---
        let t161 = t112 * t72;
        let t163 = t58 * t72;
        let t164 = t163 * t132;
        let t168 = t8 / t19 / t9;
        let t170 = t18 * t156;
        let t173 = -2.0 / 27.0 * t14 * t170 - 2.0 / 27.0 * t3 * t168;
        let t174 = param_ap * t173;
        let t175 = t174 * t83;
        let t176 = t81 * t81;
        let t178 = t23 * t23;
        let t179 = 1.0 / t178;
        let t180 = param_ap * t176 * t179;
        let t184 = -2.0 / 27.0 * t26 * t168 - 2.0 / 27.0 * t29 * t170;
        let t185 = param_af * t184;
        let t187 = t89 * t89;
        let t189 = t32 * t32;
        let t190 = 1.0 / t189;
        let t192 = -param_af * t187 * t190 + t185 * t91 - t175 + t180;
        let t193 = t192 * t56;
        let t194 = t93 * t53;
        let t195 = t194 * t109;
        let t197 = t35 * t52;
        let t198 = t109 * t109;
        let t202 = 1.0 / t43 / t39;
        let t203 = t99 * t99;
        let t206 = 1.0 / t123;
        let t207 = t36 * t206;
        let t209 = -2.0 * t97 + 2.0 * t207;
        let t213 = piecewise3(t40, 0.0, -2.0 / 9.0 * t202 * t203 + 2.0 / 3.0 * t96 * t209);
        let t215 = 1.0 / t48 / t46;
        let t216 = t104 * t104;
        let t219 = -t209;
        let t223 = piecewise3(t47, 0.0, -2.0 / 9.0 * t215 * t216 + 2.0 / 3.0 * t103 * t219);
        let t225 = t213 / 2.0 + t223 / 2.0;
        let t228 = -12.0 * t197 * t198 - 6.0 * t95 * t225 + t175 - t180 + t193 - 12.0 * t195;
        let t229 = t9 * t228;
        let t233 = t132 * t132;
        let t234 = t72 * t233;
        let t237 = 1.0 / t116 / t58;
        let t238 = param_h * t237;
        let t239 = t112 * t112;
        let t240 = t239 * t119;
        let t245 = t112 * t2;
        let t246 = t118 * t245;
        let t247 = t246 * t129;
        let t249 = t61 * t61;
        let t251 = 1.0 / t10 / t249;
        let t252 = t59 * t251;
        let t253 = t252 * t128;
        let t255 = 35.0 / 216.0 * t122 * t253;
        let t256 = t71 * t1;
        let t257 = t59 * t59;
        let t258 = t249 * t61;
        let t260 = 1.0 / t19 / t258;
        let t261 = t257 * t260;
        let t262 = t65 * t65;
        let t263 = t69 * t69;
        let t264 = 1.0 / t263;
        let t265 = t262 * t264;
        let t266 = t261 * t265;
        let t268 = 49.0 / 6912.0 * t256 * t266;
        let t269 = 2.0 * t238 * t240 - t118 * t228 * t119 + 7.0 / 72.0 * t247 + t255 - t268;
        let t270 = t72 * t269;
        let tv2rho20 = 2.0 * t113 * t133 + t115 * t234 + t115 * t270 + t229 * t72 + 2.0 * t161 + 2.0 * t164;
        v2rho2[ip * 3] += tv2rho20;
        let t272 = t147 * t72;
        let t274 = t194 * t144;
        let t276 = t144 * t109;
        let t279 = t202 * t135;
        let t282 = t96 * t36;
        let t286 = piecewise3(t40, 0.0, -2.0 / 9.0 * t279 * t99 + 4.0 / 3.0 * t282 * t206);
        let t287 = t215 * t139;
        let t290 = t103 * t36;
        let t294 = piecewise3(t47, 0.0, -2.0 / 9.0 * t287 * t104 - 4.0 / 3.0 * t290 * t206);
        let t296 = t286 / 2.0 + t294 / 2.0;
        let t299 = -12.0 * t197 * t276 - 6.0 * t95 * t296 + t175 - t180 + t193 - 6.0 * t195 - 6.0 * t274;
        let t300 = t9 * t299;
        let t303 = t163 * t152;
        let t305 = t133 * t152;
        let t310 = t299 * t119;
        let t312 = t147 * t2;
        let t313 = t118 * t312;
        let t314 = t313 * t129;
        let t317 = 2.0 * t238 * t150 * t112 - t118 * t310 + 7.0 / 144.0 * t314 + 7.0 / 144.0 * t247 + t255 - t268;
        let t318 = t72 * t317;
        let tv2rho21 = t113 * t153 + t115 * t305 + t115 * t318 + t148 * t133 + t300 * t72 + t161 + t164 + t272 + t303;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t323 = t144 * t144;
        let t326 = t135 * t135;
        let t330 = 2.0 * t97 + 2.0 * t207;
        let t334 = piecewise3(t40, 0.0, -2.0 / 9.0 * t202 * t326 + 2.0 / 3.0 * t96 * t330);
        let t335 = t139 * t139;
        let t338 = -t330;
        let t342 = piecewise3(t47, 0.0, -2.0 / 9.0 * t215 * t335 + 2.0 / 3.0 * t103 * t338);
        let t344 = t334 / 2.0 + t342 / 2.0;
        let t347 = -12.0 * t197 * t323 - 6.0 * t95 * t344 + t175 - t180 + t193 - 12.0 * t274;
        let t348 = t9 * t347;
        let t352 = t152 * t152;
        let t353 = t72 * t352;
        let t355 = t147 * t147;
        let t356 = t355 * t119;
        let t359 = t347 * t119;
        let t362 = 2.0 * t238 * t356 - t118 * t359 + 7.0 / 72.0 * t314 + t255 - t268;
        let t363 = t72 * t362;
        let tv2rho22 = t115 * t353 + t115 * t363 + 2.0 * t148 * t153 + t348 * t72 + 2.0 * t272 + 2.0 * t303;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t365 = t63 * t72;
        let t367 = t365 * param_h * t159;
        let t368 = t367 / 36.0;
        let t372 = param_h * t2 * t59 * t127;
        let t373 = t157 * t132 * t372;
        let t377 = 1.0 / t19 / t249 * t72;
        let t378 = t377 * param_h;
        let t379 = t1 * t257;
        let t381 = t379 * t264 * t65;
        let t382 = t378 * t381;
        let t383 = 7.0 / 2304.0 * t382;
        let tv2rhosigma0 = -t368 + t373 / 48.0 + t383;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let t384 = t367 / 18.0;
        let t386 = 7.0 / 1152.0 * t382;
        let tv2rhosigma1 = -t384 + t373 / 24.0 + t386;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t388 = t157 * t152 * t372;
        let tv2rhosigma3 = -t368 + t388 / 48.0 + t383;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = -t384 + t388 / 24.0 + t386;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t392 = 1.0 / t19 / t123;
        let t393 = t392 * t72;
        let t394 = param_h * param_h;
        let t397 = t257 * t264;
        let t398 = t70 * t1 * t397;
        let t401 = t379 * t264;
        let t403 = t393 * t394 * t398 - t393 * param_h * t401;
        let tv2sigma20 = t403 / 768.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = t403 / 384.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = tv2sigma20;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = t403 / 192.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = tv2sigma21;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = tv2sigma22;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
