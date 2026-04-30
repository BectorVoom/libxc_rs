//! GGA_X_EV93 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 55 shared lines across all orders.
//! Delta: 44 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ev93_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (55 lines) ---
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
        let t18 = t6 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = param_a1 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = t20 * t20;
        let t38 = param_a2 * t37;
        let t40 = 1.0 / t23 / t22;
        let t41 = t38 * t40;
        let t42 = sigma[ip] * sigma[ip];
        let t43 = t42 * t27;
        let t44 = t30 * t30;
        let t45 = t44 * rho[ip];
        let t47 = 1.0 / t19 / t45;
        let t48 = t43 * t47;
        let t51 = t22 * t22;
        let t52 = 1.0 / t51;
        let t53 = param_a3 * t52;
        let t54 = t42 * sigma[ip];
        let t55 = t44 * t44;
        let t56 = 1.0 / t55;
        let t57 = t54 * t56;
        let t60 = 1.0 + t26 * t34 / 24.0 + t41 * t48 / 288.0 + t53 * t57 / 576.0;
        let t61 = t19 * t60;
        let t62 = param_b1 * t20;
        let t63 = t62 * t25;
        let t66 = param_b2 * t37;
        let t67 = t66 * t40;
        let t70 = param_b3 * t52;
        let t73 = 1.0 + t63 * t34 / 24.0 + t67 * t48 / 288.0 + t70 * t57 / 576.0;
        let t74 = 1.0 / t73;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t61 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        // --- vxc delta (30 lines) ---
        let t79 = 1.0 / t31;
        let t80 = t79 * t60;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t87 = t29 * t86;
        let t90 = t44 * t30;
        let t92 = 1.0 / t19 / t90;
        let t93 = t43 * t92;
        let t96 = t55 * rho[ip];
        let t97 = 1.0 / t96;
        let t98 = t54 * t97;
        let t101 = -t26 * t87 / 9.0 - t41 * t93 / 54.0 - t53 * t98 / 72.0;
        let t102 = t19 * t101;
        let t106 = t73 * t73;
        let t107 = 1.0 / t106;
        let t114 = -t63 * t87 / 9.0 - t67 * t93 / 54.0 - t70 * t98 / 72.0;
        let t115 = t107 * t114;
        let t120 = piecewise3(t2, 0.0, -t18 * t80 * t74 / 8.0 - 3.0 / 8.0 * t18 * t102 * t74 + 3.0 / 8.0 * t18 * t61 * t115);
        let tvrho0 = 2.0 * rho[ip] * t120 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t123 = t25 * t28;
        let t124 = t123 * t33;
        let t127 = sigma[ip] * t27;
        let t128 = t127 * t47;
        let t131 = t42 * t56;
        let t134 = t21 * t124 / 24.0 + t41 * t128 / 144.0 + t53 * t131 / 192.0;
        let t135 = t19 * t134;
        let t144 = t62 * t124 / 24.0 + t67 * t128 / 144.0 + t70 * t131 / 192.0;
        let t145 = t107 * t144;
        let t150 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t135 * t74 + 3.0 / 8.0 * t18 * t61 * t145);
        let tvsigma0 = 2.0 * rho[ip] * t150;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (44 lines) ---
        let t154 = 1.0 / t31 / rho[ip];
        let t155 = t154 * t60;
        let t159 = t79 * t101;
        let t167 = 1.0 / t31 / t44;
        let t168 = t29 * t167;
        let t171 = t44 * t84;
        let t173 = 1.0 / t19 / t171;
        let t174 = t43 * t173;
        let t178 = 1.0 / t55 / t30;
        let t179 = t54 * t178;
        let t182 = 11.0 / 27.0 * t26 * t168 + 19.0 / 162.0 * t41 * t174 + t53 * t179 / 8.0;
        let t183 = t19 * t182;
        let t191 = 1.0 / t106 / t73;
        let t192 = t114 * t114;
        let t193 = t191 * t192;
        let t203 = 11.0 / 27.0 * t63 * t168 + 19.0 / 162.0 * t67 * t174 + t70 * t179 / 8.0;
        let t204 = t107 * t203;
        let t209 = piecewise3(t2, 0.0, t18 * t155 * t74 / 12.0 - t18 * t159 * t74 / 4.0 + t18 * t80 * t115 / 4.0 - 3.0 / 8.0 * t18 * t183 * t74 + 3.0 / 4.0 * t18 * t102 * t115 - 3.0 / 4.0 * t18 * t61 * t193 + 3.0 / 8.0 * t18 * t61 * t204);
        let tv2rho20 = 2.0 * rho[ip] * t209 + 4.0 * t120;
        v2rho2[ip] += tv2rho20;
        let t212 = t79 * t134;
        let t216 = t123 * t86;
        let t219 = t127 * t92;
        let t222 = t42 * t97;
        let t225 = -t21 * t216 / 9.0 - t41 * t219 / 27.0 - t53 * t222 / 24.0;
        let t226 = t19 * t225;
        let t240 = t6 * t17 * t19;
        let t241 = t60 * t191;
        let t242 = t144 * t114;
        let t243 = t241 * t242;
        let t252 = -t62 * t216 / 9.0 - t67 * t219 / 27.0 - t70 * t222 / 24.0;
        let t253 = t107 * t252;
        let t258 = piecewise3(t2, 0.0, -t18 * t212 * t74 / 8.0 - 3.0 / 8.0 * t18 * t226 * t74 + 3.0 / 8.0 * t18 * t135 * t115 + t18 * t80 * t145 / 8.0 + 3.0 / 8.0 * t18 * t102 * t145 - 3.0 / 4.0 * t240 * t243 + 3.0 / 8.0 * t18 * t61 * t253);
        let tv2rhosigma0 = 2.0 * rho[ip] * t258 + 2.0 * t150;
        v2rhosigma[ip] += tv2rhosigma0;
        let t261 = t40 * t27;
        let t262 = t261 * t47;
        let t265 = sigma[ip] * t56;
        let t268 = t38 * t262 / 144.0 + t53 * t265 / 96.0;
        let t269 = t19 * t268;
        let t276 = t144 * t144;
        let t277 = t191 * t276;
        let t285 = t66 * t262 / 144.0 + t70 * t265 / 96.0;
        let t286 = t107 * t285;
        let t291 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t269 * t74 + 3.0 / 4.0 * t18 * t135 * t145 - 3.0 / 4.0 * t18 * t61 * t277 + 3.0 / 8.0 * t18 * t61 * t286);
        let tv2sigma20 = 2.0 * rho[ip] * t291;
        v2sigma2[ip] += tv2sigma20;
    }
}
