//! GGA_X_MPBE fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 50 shared lines across all orders.
//! Delta: 42 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_mpbe_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (50 lines) ---
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
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = param_c1 * t20 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = param_a * t20;
        let t39 = 1.0 + t34 * t25 * t29 * t33 / 24.0;
        let t40 = 1.0 / t39;
        let t45 = t20 * t20;
        let t48 = 1.0 / t23 / t22;
        let t49 = param_c2 * t45 * t48;
        let t50 = sigma[ip] * sigma[ip];
        let t51 = t50 * t27;
        let t52 = t30 * t30;
        let t53 = t52 * rho[ip];
        let t55 = 1.0 / t18 / t53;
        let t56 = t39 * t39;
        let t57 = 1.0 / t56;
        let t58 = t55 * t57;
        let t62 = t22 * t22;
        let t63 = 1.0 / t62;
        let t64 = param_c3 * t63;
        let t65 = t50 * sigma[ip];
        let t66 = t52 * t52;
        let t67 = 1.0 / t66;
        let t69 = t56 * t39;
        let t70 = 1.0 / t69;
        let t74 = 1.0 + t26 * t29 * t33 * t40 / 24.0 + t49 * t51 * t58 / 288.0 + t64 * t65 * t67 * t70 / 576.0;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        // --- vxc delta (38 lines) ---
        let t80 = t17 / t31;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t91 = param_c1 * t45;
        let t93 = t91 * t48 * t50;
        let t94 = t52 * t30;
        let t96 = 1.0 / t18 / t94;
        let t97 = t27 * t96;
        let t98 = t57 * param_a;
        let t99 = t97 * t98;
        let t102 = t96 * t57;
        let t106 = param_c2 * t63;
        let t107 = t106 * t65;
        let t108 = t66 * rho[ip];
        let t109 = 1.0 / t108;
        let t110 = t109 * t70;
        let t111 = t110 * param_a;
        let t118 = t50 * t50;
        let t119 = t66 * t84;
        let t121 = 1.0 / t31 / t119;
        let t124 = t56 * t56;
        let t125 = 1.0 / t124;
        let t128 = t20 * t25 * t28;
        let t129 = t125 * param_a * t128;
        let t132 = -t26 * t29 * t86 * t40 / 9.0 + t93 * t99 / 108.0 - t49 * t51 * t102 / 54.0 + t107 * t111 / 108.0 - t64 * t65 * t109 * t70 / 72.0 + t64 * t118 * t121 * t129 / 1728.0;
        let t137 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t132);
        let tvrho0 = 2.0 * rho[ip] * t137 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t146 = t27 * t55;
        let t147 = t146 * t98;
        let t150 = sigma[ip] * t27;
        let t154 = t106 * t50;
        let t155 = t67 * t70;
        let t156 = t155 * param_a;
        let t163 = t66 * t30;
        let t165 = 1.0 / t31 / t163;
        let t170 = t26 * t28 * t33 * t40 / 24.0 - t91 * t48 * sigma[ip] * t147 / 288.0 + t49 * t150 * t58 / 144.0 - t154 * t156 / 288.0 + t64 * t50 * t67 * t70 / 192.0 - t64 * t65 * t165 * t129 / 4608.0;
        let t174 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t170);
        let tvsigma0 = 2.0 * rho[ip] * t174;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (42 lines) ---
        let t179 = t17 / t31 / rho[ip];
        let t187 = 1.0 / t31 / t52;
        let t192 = t52 * t84;
        let t194 = 1.0 / t18 / t192;
        let t195 = t27 * t194;
        let t196 = t195 * t98;
        let t199 = param_c1 * t63;
        let t200 = t199 * t65;
        let t201 = 1.0 / t163;
        let t202 = t201 * t70;
        let t203 = param_a * param_a;
        let t207 = t194 * t57;
        let t211 = t202 * param_a;
        let t214 = t66 * t52;
        let t216 = 1.0 / t31 / t214;
        let t217 = t118 * t216;
        let t220 = t125 * t203 * t128;
        let t230 = t118 * sigma[ip];
        let t233 = 1.0 / t18 / t66 / t192;
        let t237 = 1.0 / t124 / t39;
        let t240 = t45 * t48 * t27;
        let t241 = t237 * t203 * t240;
        let t244 = 11.0 / 27.0 * t26 * t29 * t187 * t40 - t93 * t196 / 12.0 + 2.0 / 81.0 * t200 * t202 * t203 + 19.0 / 162.0 * t49 * t51 * t207 - 43.0 / 324.0 * t107 * t211 + t106 * t217 * t220 / 324.0 + t64 * t65 * t201 * t70 / 8.0 - 59.0 / 5184.0 * t64 * t217 * t129 + t64 * t230 * t233 * t241 / 1944.0;
        let t249 = piecewise3(t2, 0.0, t6 * t179 * t74 / 12.0 - t6 * t80 * t132 / 4.0 - 3.0 / 8.0 * t6 * t19 * t244);
        let tv2rho20 = 2.0 * rho[ip] * t249 + 4.0 * t137;
        v2rho2[ip] += tv2rho20;
        let t260 = t91 * t48 * t27;
        let t261 = param_a * sigma[ip];
        let t266 = t110 * t203;
        let t274 = t65 * t121;
        let t285 = t66 * t94;
        let t287 = 1.0 / t18 / t285;
        let t292 = -t26 * t28 * t86 * t40 / 9.0 + t260 * t102 * t261 / 36.0 - t199 * t50 * t266 / 108.0 - t49 * t150 * t102 / 27.0 + 5.0 / 108.0 * t154 * t111 - t106 * t274 * t220 / 864.0 - t64 * t50 * t109 * t70 / 24.0 + 7.0 / 1728.0 * t64 * t274 * t129 - t64 * t118 * t287 * t241 / 5184.0;
        let t297 = piecewise3(t2, 0.0, -t6 * t80 * t170 / 8.0 - 3.0 / 8.0 * t6 * t19 * t292);
        let tv2rhosigma0 = 2.0 * rho[ip] * t297 + 2.0 * t174;
        v2rhosigma[ip] += tv2rhosigma0;
        let t300 = t91 * t48;
        let t304 = t155 * t203;
        let t313 = t50 * t165;
        let t324 = t66 * t53;
        let t326 = 1.0 / t18 / t324;
        let t331 = -t300 * t147 / 144.0 + t199 * sigma[ip] * t304 / 288.0 + t49 * t146 * t57 / 144.0 - t106 * sigma[ip] * t156 / 72.0 + t106 * t313 * t220 / 2304.0 + t64 * sigma[ip] * t67 * t70 / 96.0 - t64 * t313 * t129 / 768.0 + t64 * t65 * t326 * t241 / 13824.0;
        let t335 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t331);
        let tv2sigma20 = 2.0 * rho[ip] * t335;
        v2sigma2[ip] += tv2sigma20;
    }
}
