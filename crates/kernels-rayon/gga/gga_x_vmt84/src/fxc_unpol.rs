//! GGA_X_VMT84 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_vmt84.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_vmt84_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_mu: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t25 * sigma[ip];
        let t27 = t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t32 = t31 * t30;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t36 = param_alpha * t20 * t25;
        let t37 = sigma[ip] * t29;
        let t38 = t37 * t33;
        let t41 = rmath::exp(-t36 * t38 / 24.0);
        let t42 = t21 * t25;
        let t45 = 1.0 + t42 * t38 / 24.0;
        let t46 = 1.0 / t45;
        let t47 = t41 * t46;
        let t48 = t34 * t47;
        let t51 = t20 * t20;
        let t54 = 1.0 / t23 / t22;
        let t55 = param_alpha * t51 * t54;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = t56 * t28;
        let t58 = t30 * t30;
        let t59 = t58 * rho[ip];
        let t61 = 1.0 / t18 / t59;
        let t65 = rmath::exp(-t55 * t57 * t61 / 288.0);
        let t68 = (1.0 - t65) * t51 * t24;
        let t69 = 1.0 / sigma[ip];
        let t70 = t69 * t28;
        let t74 = t27 * t48 / 24.0 + 2.0 * t68 * t70 * t32 + t65;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t17 / t31;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t88 = t29 * t86 * t47;
        let t91 = param_mu * t51;
        let t92 = t54 * t56;
        let t93 = t91 * t92;
        let t94 = t58 * t30;
        let t96 = 1.0 / t18 / t94;
        let t97 = t28 * t96;
        let t98 = param_alpha * t41;
        let t99 = t98 * t46;
        let t103 = param_mu * param_mu;
        let t104 = t103 * t51;
        let t105 = t104 * t92;
        let t106 = t45 * t45;
        let t107 = 1.0 / t106;
        let t108 = t41 * t107;
        let t109 = t97 * t108;
        let t112 = t86 * t65;
        let t116 = t31 * rho[ip];
        let t120 = t96 * t65;
        let t124 = -t27 * t88 / 9.0 + t93 * t97 * t99 / 108.0 + t105 * t109 / 108.0 - 2.0 / 9.0 * t36 * t37 * t112 + 16.0 / 3.0 * t68 * t70 * t116 + t55 * t57 * t120 / 54.0;
        let t129 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t124);
        let tvrho0 = 2.0 * rho[ip] * t129 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t134 = t54 * sigma[ip];
        let t136 = t28 * t61;
        let t141 = t136 * t108;
        let t147 = 1.0 / t56;
        let t148 = t147 * t28;
        let t152 = sigma[ip] * t28;
        let t157 = t42 * t48 / 24.0 - t91 * t134 * t136 * t99 / 288.0 - t104 * t134 * t141 / 288.0 + t36 * t34 * t65 / 12.0 - 2.0 * t68 * t148 * t32 - t55 * t152 * t61 * t65 / 144.0;
        let t161 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t157);
        let tvsigma0 = 2.0 * rho[ip] * t161;
        vsigma[ip] += tvsigma0;
        let t165 = t17 / t116;
        let t173 = 1.0 / t31 / t58;
        let t174 = t29 * t173;
        let t175 = t174 * t47;
        let t178 = t58 * t84;
        let t180 = 1.0 / t18 / t178;
        let t181 = t28 * t180;
        let t185 = t181 * t108;
        let t188 = t22 * t22;
        let t189 = 1.0 / t188;
        let t190 = param_mu * t189;
        let t191 = t56 * sigma[ip];
        let t192 = t190 * t191;
        let t193 = t58 * t58;
        let t194 = t193 * t30;
        let t195 = 1.0 / t194;
        let t196 = param_alpha * param_alpha;
        let t201 = t103 * t189;
        let t202 = t201 * t191;
        let t208 = t103 * param_mu * t189;
        let t209 = t208 * t191;
        let t212 = 1.0 / t106 / t45;
        let t216 = t173 * t65;
        let t220 = t196 * t189;
        let t228 = t180 * t65;
        let t234 = 1.0 / t24 / t188;
        let t235 = t196 * t20 * t234;
        let t236 = t56 * t56;
        let t237 = t236 * t29;
        let t238 = t193 * t58;
        let t240 = 1.0 / t31 / t238;
        let t241 = t240 * t65;
        let t245 = 11.0 / 27.0 * t27 * t175 - t93 * t181 * t99 / 12.0 - t105 * t185 / 12.0 + t192 * t195 * t196 * t47 / 81.0 + 2.0 / 81.0 * t202 * t195 * param_alpha * t108 + 2.0 / 81.0 * t209 * t195 * t41 * t212 + 2.0 / 9.0 * t36 * t37 * t216 - 4.0 / 81.0 * t220 * t191 * t195 * t65 + 80.0 / 9.0 * t68 * t70 * t31 - 19.0 / 162.0 * t55 * t57 * t228 + t235 * t237 * t241 / 486.0;
        let t250 = piecewise3(t2, 0.0, t6 * t165 * t74 / 12.0 - t6 * t80 * t124 / 4.0 - 3.0 / 8.0 * t6 * t19 * t245);
        let tv2rho20 = 2.0 * rho[ip] * t250 + 4.0 * t129;
        v2rho2[ip] += tv2rho20;
        let t258 = t54 * t28;
        let t259 = t91 * t258;
        let t260 = t96 * param_alpha;
        let t262 = sigma[ip] * t41 * t46;
        let t266 = t104 * t258;
        let t268 = t107 * sigma[ip];
        let t273 = t193 * rho[ip];
        let t274 = 1.0 / t273;
        let t286 = t274 * t41 * t212;
        let t299 = t191 * t29;
        let t300 = t193 * t84;
        let t302 = 1.0 / t31 / t300;
        let t307 = -t42 * t88 / 9.0 + t259 * t260 * t262 / 36.0 + t266 * t96 * t41 * t268 / 36.0 - t190 * t56 * t274 * t196 * t47 / 216.0 - t201 * t56 * t274 * param_alpha * t108 / 108.0 - t208 * t56 * t286 / 108.0 + t220 * t274 * t56 * t65 / 54.0 - 16.0 / 3.0 * t68 * t148 * t116 + t55 * t152 * t120 / 27.0 - t235 * t299 * t302 * t65 / 1296.0;
        let t312 = piecewise3(t2, 0.0, -t6 * t80 * t157 / 8.0 - 3.0 / 8.0 * t6 * t19 * t307);
        let tv2rhosigma0 = 2.0 * rho[ip] * t312 + 2.0 * t161;
        v2rhosigma[ip] += tv2rhosigma0;
        let t319 = t104 * t54;
        let t323 = 1.0 / t193;
        let t335 = t323 * t41 * t212;
        let t342 = t69 * t29;
        let t343 = t33 * t65;
        let t347 = 1.0 / t191;
        let t348 = t347 * t28;
        let t357 = 1.0 / t31 / t194;
        let t362 = -t259 * t61 * param_alpha * t47 / 144.0 - t319 * t141 / 144.0 + t190 * sigma[ip] * t323 * t196 * t47 / 576.0 + t201 * sigma[ip] * t323 * param_alpha * t108 / 288.0 + t208 * sigma[ip] * t335 / 288.0 - t220 * t323 * sigma[ip] * t65 / 144.0 - t36 * t342 * t343 / 12.0 + 4.0 * t68 * t348 * t32 - t55 * t136 * t65 / 144.0 + t235 * t56 * t29 * t357 * t65 / 3456.0;
        let t366 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t362);
        let tv2sigma20 = 2.0 * rho[ip] * t366;
        v2sigma2[ip] += tv2sigma20;
    }
}
