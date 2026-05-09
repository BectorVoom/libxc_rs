//! MGGA_X_2D_PRHG07 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 27 shared lines across all orders.
//! Delta: 126 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_2d_prhg07_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (27 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = f64::sqrt(zeta_threshold);
        let t12 = f64::sqrt(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = M_PI * t14;
        let t16 = M_SQRT2;
        let t17 = f64::sqrt(rho[ip]);
        let t18 = t16 * t17;
        let t19 = rho[ip] * rho[ip];
        let t20 = 1.0 / t19;
        let t25 = t19 * rho[ip];
        let t26 = 1.0 / t25;
        let t30 = 1.0 / M_PI;
        let t31 = (lapl[ip] * t20 / 2.0 - 2.0 * tau[ip] * t20 + sigma[ip] * t26 / 4.0) * t30;
        let t32 = -0.9999999999e0 < t31;
        let t33 = piecewise3(t32, t31, -0.9999999999e0);
        let t34 = f64::exp(-1.0);
        let t36 = lambert_w(t33 * t34);
        let t37 = t36 + 1.0;
        let t38 = t37 / 2.0;
        let t39 = xc_bessel_I0(t38);
        let t43 = piecewise3(t3, 0.0, -t15 * t18 * t39 / 8.0);
        let tzk0 = 2.0 * t43;
        zk[ip] += tzk0;
        // --- vxc delta (29 lines) ---
        let t45 = t16 / t17;
        let t48 = t15 * t18;
        let t49 = xc_bessel_I1(t38);
        let t53 = t19 * t19;
        let t54 = 1.0 / t53;
        let t59 = piecewise3(t32, (-lapl[ip] * t26 + 4.0 * tau[ip] * t26 - 3.0 / 4.0 * sigma[ip] * t54) * t30, 0.0);
        let t61 = 1.0 / t37;
        let t62 = t36 * t61;
        let t64 = t62 / t33;
        let t65 = t49 * t59 * t64;
        let t69 = piecewise3(t3, 0.0, -t15 * t45 * t39 / 16.0 - t48 * t65 / 16.0);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t43;
        vrho[ip] += tvrho0;
        let t72 = t26 * t30;
        let t74 = piecewise3(t32, t72 / 4.0, 0.0);
        let t75 = t49 * t74;
        let t76 = t75 * t64;
        let t79 = piecewise3(t3, 0.0, -t48 * t76 / 16.0);
        let tvsigma0 = 2.0 * rho[ip] * t79;
        vsigma[ip] += tvsigma0;
        let t81 = t20 * t30;
        let t83 = piecewise3(t32, t81 / 2.0, 0.0);
        let t84 = t49 * t83;
        let t85 = t84 * t64;
        let t88 = piecewise3(t3, 0.0, -t48 * t85 / 16.0);
        let tvlapl0 = 2.0 * rho[ip] * t88;
        vlapl[ip] += tvlapl0;
        let t91 = piecewise3(t32, -2.0 * t81, 0.0);
        let t92 = t49 * t91;
        let t93 = t92 * t64;
        let t96 = piecewise3(t3, 0.0, -t48 * t93 / 16.0);
        let tvtau0 = 2.0 * rho[ip] * t96;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (126 lines) ---
        let t101 = t16 / t17 / rho[ip];
        let t105 = t15 * t45;
        let t108 = 1.0 / t38;
        let t110 = -t108 * t49 + t39;
        let t111 = t59 * t59;
        let t113 = t36 * t36;
        let t114 = t37 * t37;
        let t115 = 1.0 / t114;
        let t116 = t113 * t115;
        let t117 = t33 * t33;
        let t118 = 1.0 / t117;
        let t119 = t116 * t118;
        let t120 = t110 * t111 * t119;
        let t128 = 1.0 / t53 / rho[ip];
        let t133 = piecewise3(t32, (3.0 * lapl[ip] * t54 + 3.0 * t128 * sigma[ip] - 12.0 * t54 * tau[ip]) * t30, 0.0);
        let t135 = t49 * t133 * t64;
        let t138 = t49 * t111;
        let t139 = t36 * t115;
        let t140 = t139 * t118;
        let t141 = t138 * t140;
        let t144 = t114 * t37;
        let t145 = 1.0 / t144;
        let t146 = t113 * t145;
        let t147 = t146 * t118;
        let t148 = t138 * t147;
        let t151 = t62 * t118;
        let t152 = t138 * t151;
        let t156 = piecewise3(t3, 0.0, t15 * t101 * t39 / 32.0 - t105 * t65 / 16.0 - t48 * t120 / 32.0 - t48 * t135 / 16.0 - t48 * t141 / 16.0 + t48 * t148 / 16.0 + t48 * t152 / 16.0);
        let tv2rho20 = 2.0 * rho[ip] * t156 + 4.0 * t69;
        v2rho2[ip] += tv2rho20;
        let t162 = t15 * t18 * t110;
        let t163 = t59 * t113;
        let t164 = t115 * t118;
        let t165 = t164 * t74;
        let t166 = t163 * t165;
        let t169 = t54 * t30;
        let t171 = piecewise3(t32, -3.0 / 4.0 * t169, 0.0);
        let t173 = t49 * t171 * t64;
        let t177 = t15 * t18 * t49;
        let t178 = t74 * t59;
        let t179 = t178 * t140;
        let t182 = t74 * t113;
        let t183 = t145 * t118;
        let t184 = t183 * t59;
        let t185 = t182 * t184;
        let t188 = t74 * t36;
        let t189 = t61 * t118;
        let t190 = t189 * t59;
        let t191 = t188 * t190;
        let t195 = piecewise3(t3, 0.0, -t105 * t76 / 32.0 - t162 * t166 / 32.0 - t48 * t173 / 16.0 - t177 * t179 / 16.0 + t177 * t185 / 16.0 + t177 * t191 / 16.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t195 + 2.0 * t79;
        v2rhosigma[ip] += tv2rhosigma0;
        let t200 = t164 * t83;
        let t201 = t163 * t200;
        let t204 = piecewise3(t32, -t72, 0.0);
        let t206 = t49 * t204 * t64;
        let t209 = t83 * t59;
        let t210 = t209 * t140;
        let t213 = t83 * t113;
        let t214 = t213 * t184;
        let t217 = t83 * t36;
        let t218 = t217 * t190;
        let t222 = piecewise3(t3, 0.0, -t105 * t85 / 32.0 - t162 * t201 / 32.0 - t48 * t206 / 16.0 - t177 * t210 / 16.0 + t177 * t214 / 16.0 + t177 * t218 / 16.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t222 + 2.0 * t88;
        v2rholapl[ip] += tv2rholapl0;
        let t227 = t164 * t91;
        let t228 = t163 * t227;
        let t232 = piecewise3(t32, 4.0 * t72, 0.0);
        let t234 = t49 * t232 * t64;
        let t237 = t91 * t59;
        let t238 = t237 * t140;
        let t241 = t91 * t113;
        let t242 = t241 * t184;
        let t245 = t91 * t36;
        let t246 = t245 * t190;
        let t250 = piecewise3(t3, 0.0, -t105 * t93 / 32.0 - t162 * t228 / 32.0 - t48 * t234 / 16.0 - t177 * t238 / 16.0 + t177 * t242 / 16.0 + t177 * t246 / 16.0);
        let tv2rhotau0 = 2.0 * rho[ip] * t250 + 2.0 * t96;
        v2rhotau[ip] += tv2rhotau0;
        let t253 = t74 * t74;
        let t255 = t110 * t253 * t119;
        let t258 = piecewise3(t32, 0.0, 0.0);
        let t260 = t49 * t258 * t64;
        let t262 = t48 * t260 / 16.0;
        let t263 = t49 * t253;
        let t264 = t263 * t140;
        let t267 = t263 * t147;
        let t270 = t263 * t151;
        let t274 = piecewise3(t3, 0.0, -t48 * t255 / 32.0 - t262 - t48 * t264 / 16.0 + t48 * t267 / 16.0 + t48 * t270 / 16.0);
        let tv2sigma20 = 2.0 * rho[ip] * t274;
        v2sigma2[ip] += tv2sigma20;
        let t276 = t182 * t200;
        let t279 = t83 * t74;
        let t280 = t279 * t140;
        let t283 = t183 * t74;
        let t284 = t213 * t283;
        let t287 = t189 * t74;
        let t288 = t217 * t287;
        let t292 = piecewise3(t3, 0.0, -t162 * t276 / 32.0 - t262 - t177 * t280 / 16.0 + t177 * t284 / 16.0 + t177 * t288 / 16.0);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t292;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t294 = t182 * t227;
        let t297 = t91 * t74;
        let t298 = t297 * t140;
        let t301 = t241 * t283;
        let t304 = t245 * t287;
        let t308 = piecewise3(t3, 0.0, -t162 * t294 / 32.0 - t262 - t177 * t298 / 16.0 + t177 * t301 / 16.0 + t177 * t304 / 16.0);
        let tv2sigmatau0 = 2.0 * rho[ip] * t308;
        v2sigmatau[ip] += tv2sigmatau0;
        let t310 = t83 * t83;
        let t312 = t110 * t310 * t119;
        let t315 = t49 * t310;
        let t316 = t315 * t140;
        let t319 = t315 * t147;
        let t322 = t315 * t151;
        let t326 = piecewise3(t3, 0.0, -t48 * t312 / 32.0 - t262 - t48 * t316 / 16.0 + t48 * t319 / 16.0 + t48 * t322 / 16.0);
        let tv2lapl20 = 2.0 * rho[ip] * t326;
        v2lapl2[ip] += tv2lapl20;
        let t328 = t213 * t227;
        let t331 = t91 * t83;
        let t332 = t331 * t140;
        let t335 = t183 * t83;
        let t336 = t241 * t335;
        let t339 = t189 * t83;
        let t340 = t245 * t339;
        let t344 = piecewise3(t3, 0.0, -t162 * t328 / 32.0 - t262 - t177 * t332 / 16.0 + t177 * t336 / 16.0 + t177 * t340 / 16.0);
        let tv2lapltau0 = 2.0 * rho[ip] * t344;
        v2lapltau[ip] += tv2lapltau0;
        let t346 = t91 * t91;
        let t348 = t110 * t346 * t119;
        let t351 = t49 * t346;
        let t352 = t351 * t140;
        let t355 = t351 * t147;
        let t358 = t351 * t151;
        let t362 = piecewise3(t3, 0.0, -t48 * t348 / 32.0 - t262 - t48 * t352 / 16.0 + t48 * t355 / 16.0 + t48 * t358 / 16.0);
        let tv2tau20 = 2.0 * rho[ip] * t362;
        v2tau2[ip] += tv2tau20;
    }
}
