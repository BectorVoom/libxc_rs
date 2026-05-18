//! GGA_K_MEYER fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_meyer_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3::<f64>(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3::<f64>(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3::<f64>(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3::<f64>(t25);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t39 = 1.0 - t29 * t32 * t35 / 864.0;
        let t40 = t24 * t24;
        let t41 = 1.0 / t26;
        let t42 = t40 * t41;
        let t43 = f64::sqrt(sigma[ip]);
        let t44 = t43 * t30;
        let t45 = t21 * rho[ip];
        let t46 = 1.0 / t45;
        let t49 = t42 * t44 * t46 / 72.0;
        let t50 = 1.0 + t49;
        let t51 = 1.0 - t49;
        let t52 = f64::abs(t51);
        let t53 = 1.0 / t52;
        let t55 = f64::ln(t50 * t53);
        let t57 = t39 * t55 * t24;
        let t58 = 1.0 / t43;
        let t59 = t26 * t58;
        let t60 = t31 * t45;
        let t63 = 3.0 / 2.0 * t57 * t59 * t60;
        let t64 = 1.0 / 2.0 - t63;
        let t65 = 1.0 / 2.0 + t63;
        let t66 = 1.0 / t65;
        let t69 = 20.0 * t64 * t66 + 1.0;
        let t73 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t69);
        let tzk0 = 2.0 * t73;
        zk[ip] += tzk0;
        let t75 = t20 / t21;
        let t79 = t42 * t43;
        let t81 = 1.0 / t21 / t33;
        let t82 = t30 * t81;
        let t83 = t82 * t55;
        let t86 = t82 * t53;
        let t88 = t52 * t52;
        let t89 = 1.0 / t88;
        let t90 = t50 * t89;
        let t91 = t90 * t42;
        let t92 = f64::abs(t51) / t51;
        let t93 = t81 * t92;
        let t97 = -t91 * t44 * t93 / 54.0 - t79 * t86 / 54.0;
        let t98 = t39 * t97;
        let t99 = 1.0 / t50;
        let t100 = t99 * t52;
        let t101 = t98 * t100;
        let t102 = t24 * t26;
        let t103 = t58 * t31;
        let t105 = t102 * t103 * t45;
        let t108 = t31 * t21;
        let t112 = -t79 * t83 / 108.0 - 3.0 / 2.0 * t101 * t105 - 2.0 * t57 * t59 * t108;
        let t114 = t65 * t65;
        let t115 = 1.0 / t114;
        let t116 = t64 * t115;
        let t117 = -t112;
        let t120 = 20.0 * t112 * t66 - 20.0 * t116 * t117;
        let t125 = piecewise3::<f64>(t2, 0.0, t7 * t75 * t69 / 10.0 + 3.0 / 20.0 * t7 * t23 * t120);
        let tvrho0 = 2.0 * rho[ip] * t125 + 2.0 * t73;
        vrho[ip] += tvrho0;
        let t128 = t42 * t30;
        let t129 = t46 * t55;
        let t133 = t42 * t58;
        let t134 = t30 * t46;
        let t135 = t134 * t53;
        let t137 = t58 * t30;
        let t138 = t46 * t92;
        let t142 = t91 * t137 * t138 / 144.0 + t133 * t135 / 144.0;
        let t143 = t39 * t142;
        let t144 = t143 * t100;
        let t147 = t43 * sigma[ip];
        let t148 = 1.0 / t147;
        let t149 = t26 * t148;
        let t153 = t128 * t129 * t58 / 288.0 - 3.0 / 2.0 * t144 * t105 + 3.0 / 4.0 * t57 * t149 * t60;
        let t155 = -t153;
        let t158 = -20.0 * t116 * t155 + 20.0 * t153 * t66;
        let t162 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t158);
        let tvsigma0 = 2.0 * rho[ip] * t162;
        vsigma[ip] += tvsigma0;
        let t165 = t20 * t46;
        let t172 = t33 * rho[ip];
        let t174 = 1.0 / t21 / t172;
        let t175 = t30 * t174;
        let t176 = t175 * t55;
        let t179 = t42 * t44;
        let t184 = t175 * t53;
        let t187 = t29 * sigma[ip];
        let t188 = t33 * t33;
        let t190 = 1.0 / t22 / t188;
        let t191 = t31 * t190;
        let t192 = t89 * t92;
        let t193 = t191 * t192;
        let t197 = 1.0 / t88 / t52;
        let t198 = t50 * t197;
        let t199 = t198 * t29;
        let t200 = t92 * t92;
        let t201 = t190 * t200;
        let t205 = t174 * t92;
        let t209 = t90 * t29;
        let t210 = 0.0;
        let t211 = t190 * t210;
        let t214 = t209 * t32 * t211 / 486.0;
        let t215 = 7.0 / 162.0 * t79 * t184 + t187 * t193 / 243.0 + t199 * t32 * t201 / 243.0 + 7.0 / 162.0 * t91 * t44 * t205 - t214;
        let t216 = t39 * t215;
        let t217 = t216 * t100;
        let t220 = t50 * t50;
        let t221 = 1.0 / t220;
        let t222 = t221 * t52;
        let t223 = 1.0 / rho[ip];
        let t224 = t222 * t223;
        let t228 = t99 * t223 * t92;
        let t232 = t102 * t103 * t21;
        let t235 = 1.0 / t22;
        let t236 = t31 * t235;
        let t240 = t79 * t176 / 108.0 - t179 * t81 * t97 * t100 / 54.0 - 3.0 / 2.0 * t217 * t105 - t98 * t224 / 3.0 - t98 * t228 / 3.0 - 4.0 * t101 * t232 - 2.0 / 3.0 * t57 * t59 * t236;
        let t243 = t112 * t115;
        let t247 = 1.0 / t114 / t65;
        let t248 = t64 * t247;
        let t249 = t117 * t117;
        let t252 = -t240;
        let t255 = -20.0 * t116 * t252 - 40.0 * t243 * t117 + 20.0 * t240 * t66 + 40.0 * t248 * t249;
        let t260 = piecewise3::<f64>(t2, 0.0, -t7 * t165 * t69 / 30.0 + t7 * t75 * t120 / 5.0 + 3.0 / 20.0 * t7 * t23 * t255);
        let tv2rho20 = 2.0 * rho[ip] * t260 + 4.0 * t125;
        v2rho2[ip] += tv2rho20;
        let t266 = t42 * t134;
        let t267 = t97 * t99;
        let t268 = t52 * t58;
        let t273 = t81 * t142 * t100;
        let t278 = t29 * t31;
        let t280 = 1.0 / t22 / t172;
        let t285 = t198 * t24;
        let t286 = t28 * t31;
        let t287 = t280 * t200;
        let t294 = t90 * t24;
        let t295 = t280 * t210;
        let t298 = t294 * t286 * t295 / 1296.0;
        let t299 = -t133 * t86 / 108.0 - t278 * t280 * t89 * t92 / 648.0 - t285 * t286 * t287 / 648.0 - t91 * t137 * t93 / 108.0 + t298;
        let t300 = t39 * t299;
        let t301 = t300 * t100;
        let t310 = t148 * t31;
        let t312 = t102 * t310 * t45;
        let t317 = t266 * t267 * t268 / 288.0 - t179 * t273 / 108.0 - 3.0 / 2.0 * t301 * t105 - t143 * t224 / 3.0 - t143 * t228 / 3.0 - 2.0 * t144 * t232 + 3.0 / 4.0 * t101 * t312 + t57 * t149 * t108;
        let t320 = t153 * t115;
        let t325 = t155 * t117;
        let t328 = -t317;
        let t331 = -20.0 * t116 * t328 - 20.0 * t320 * t117 - 20.0 * t243 * t155 + 40.0 * t248 * t325 + 20.0 * t317 * t66;
        let t336 = piecewise3::<f64>(t2, 0.0, t7 * t75 * t158 / 10.0 + 3.0 / 20.0 * t7 * t23 * t331);
        let tv2rhosigma0 = 2.0 * rho[ip] * t336 + 2.0 * t162;
        v2rhosigma[ip] += tv2rhosigma0;
        let t339 = t142 * t99;
        let t346 = t42 * t148;
        let t349 = 1.0 / sigma[ip];
        let t350 = t29 * t349;
        let t351 = t31 * t35;
        let t352 = t351 * t192;
        let t355 = t349 * t31;
        let t356 = t35 * t200;
        let t360 = t148 * t30;
        let t364 = t35 * t210;
        let t367 = t209 * t355 * t364 / 3456.0;
        let t368 = -t346 * t135 / 288.0 + t350 * t352 / 1728.0 + t199 * t355 * t356 / 1728.0 - t91 * t360 * t138 / 288.0 - t367;
        let t369 = t39 * t368;
        let t370 = t369 * t100;
        let t373 = t222 * t349;
        let t377 = t99 * t349 * t92;
        let t382 = sigma[ip] * sigma[ip];
        let t384 = 1.0 / t43 / t382;
        let t385 = t26 * t384;
        let t389 = t266 * t339 * t268 / 144.0 - t128 * t129 * t148 / 288.0 - 3.0 / 2.0 * t370 * t105 + t143 * t373 / 8.0 + t143 * t377 / 8.0 + 3.0 / 2.0 * t144 * t312 - 9.0 / 8.0 * t57 * t385 * t60;
        let t394 = t155 * t155;
        let t397 = -t389;
        let t400 = -20.0 * t116 * t397 - 40.0 * t320 * t155 + 40.0 * t248 * t394 + 20.0 * t389 * t66;
        let t404 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t400);
        let tv2sigma20 = 2.0 * rho[ip] * t404;
        v2sigma2[ip] += tv2sigma20;
    }
}
