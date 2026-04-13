//! MGGA_K_LK vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_lk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_lk_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = t25 * t25;
        let t42 = 1.0 / t27 / t26;
        let t43 = t40 * t42;
        let t44 = lapl[ip] * lapl[ip];
        let t45 = t44 * t31;
        let t46 = t34 * rho[ip];
        let t48 = 1.0 / t22 / t46;
        let t51 = t43 * t45 * t48 / 2916.0;
        let t52 = t43 * sigma[ip];
        let t53 = t34 * t34;
        let t55 = 1.0 / t22 / t53;
        let t56 = t31 * t55;
        let t57 = t56 * lapl[ip];
        let t59 = t52 * t57 / 2592.0;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t60 * t31;
        let t62 = t53 * rho[ip];
        let t64 = 1.0 / t22 / t62;
        let t67 = t43 * t61 * t64 / 8748.0;
        let t68 = t43 * t60;
        let t69 = t31 * t64;
        let t70 = 1.0 / param_kappa;
        let t71 = t69 * t70;
        let t76 = 1.0 + (5.0 / 648.0 * t30 * t33 * t36 + t51 - t59 + t67 + 25.0 / 209952.0 * t68 * t71) * t70;
        let t78 = t30 * sigma[ip];
        let t79 = t32 * t36;
        let t80 = t51 - t59 + t67;
        let t81 = t80 * t70;
        let t85 = t26 * t26;
        let t86 = 1.0 / t85;
        let t87 = t60 * sigma[ip];
        let t88 = t86 * t87;
        let t89 = t53 * t53;
        let t90 = 1.0 / t89;
        let t91 = param_kappa * param_kappa;
        let t92 = 1.0 / t91;
        let t93 = t90 * t92;
        let t98 = 1.0 + (5.0 / 324.0 * t78 * t79 * t81 + 125.0 / 0.11337408e8 * t88 * t93) * t70;
        let t102 = 1.0 + param_kappa * (2.0 - 1.0 / t76 - 1.0 / t98);
        let t106 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t21 * t23 * t102);
        let tzk0 = 2.0 * t106;
        zk[ip] += tzk0;
        let t107 = 1.0 / t22;
        let t112 = t8 * t21;
        let t113 = t23 * param_kappa;
        let t114 = t76 * t76;
        let t115 = 1.0 / t114;
        let t117 = 1.0 / t23 / t46;
        let t123 = 5.0 / 4374.0 * t43 * t45 * t55;
        let t124 = t69 * lapl[ip];
        let t126 = 13.0 / 7776.0 * t52 * t124;
        let t127 = t53 * t34;
        let t129 = 1.0 / t22 / t127;
        let t132 = 4.0 / 6561.0 * t43 * t61 * t129;
        let t133 = t31 * t129;
        let t134 = t133 * t70;
        let t137 = -5.0 / 243.0 * t30 * t33 * t117 - t123 + t126 - t132 - 25.0 / 39366.0 * t68 * t134;
        let t140 = t98 * t98;
        let t141 = 1.0 / t140;
        let t142 = t32 * t117;
        let t146 = -t123 + t126 - t132;
        let t147 = t146 * t70;
        let t151 = t89 * rho[ip];
        let t152 = 1.0 / t151;
        let t153 = t152 * t92;
        let t156 = -10.0 / 243.0 * t78 * t142 * t81 + 5.0 / 324.0 * t78 * t79 * t147 - 125.0 / 1417176.0 * t88 * t153;
        let t159 = t115 * t137 * t70 + t141 * t156 * t70;
        let t164 = piecewise3(t3, 0.0, t8 * t21 * t107 * t102 / 10.0 + 3.0 / 20.0 * t112 * t113 * t159);
        let tvrho0 = 2.0 * rho[ip] * t164 + 2.0 * t106;
        vrho[ip] += tvrho0;
        let t169 = t43 * t57;
        let t170 = t169 / 2592.0;
        let t171 = sigma[ip] * t31;
        let t173 = t43 * t171 * t64;
        let t174 = t173 / 4374.0;
        let t177 = 5.0 / 648.0 * t30 * t79 - t170 + t174 + 25.0 / 104976.0 * t52 * t71;
        let t180 = t30 * t32;
        let t185 = -t170 + t174;
        let t186 = t185 * t70;
        let t190 = t86 * t60;
        let t193 = 5.0 / 324.0 * t180 * t36 * t80 * t70 + 5.0 / 324.0 * t78 * t79 * t186 + 125.0 / 3779136.0 * t190 * t93;
        let t196 = t115 * t177 * t70 + t141 * t193 * t70;
        let t200 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t196);
        let tvsigma0 = 2.0 * rho[ip] * t200;
        vsigma[ip] += tvsigma0;
        let t209 = t43 * lapl[ip] * t31 * t48 / 1458.0 - t43 * t171 * t55 / 2592.0;
        let t212 = t141 * t25;
        let t213 = t29 * sigma[ip];
        let t214 = t212 * t213;
        let t215 = t209 * t92;
        let t216 = t79 * t215;
        let t219 = t115 * t209 * t70 + 5.0 / 324.0 * t214 * t216;
        let t223 = piecewise3(t3, 0.0, 3.0 / 20.0 * t112 * t113 * t219);
        let tvlapl0 = 2.0 * rho[ip] * t223;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
