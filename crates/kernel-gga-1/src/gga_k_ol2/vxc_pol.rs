//! GGA_K_OL2 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 61 shared lines across all orders.
//! Delta: 54 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_ol2_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        // --- shared preamble (61 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = param_bb * sigma0;
        let t33 = rho0 * rho0;
        let t34 = pow_1_3(rho0);
        let t35 = t34 * t34;
        let t37 = 1.0 / t35 / t33;
        let t40 = f64::sqrt(sigma0);
        let t41 = param_cc * t40;
        let t43 = 1.0 / t34 / rho0;
        let t44 = M_CBRT2;
        let t47 = 4.0 * t40 * t43 + t44;
        let t48 = 1.0 / t47;
        let t49 = t43 * t48;
        let t51 = param_aa + 0.13888888888888888889e-1 * t32 * t37 + t41 * t49;
        let t55 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t51);
        let t56 = rho1 <= dens_threshold;
        let t57 = -t17;
        let t59 = piecewise5(t15, t12, t11, t16, t57 * t8);
        let t60 = 1.0 + t59;
        let t61 = t60 <= zeta_threshold;
        let t62 = pow_1_3(t60);
        let t63 = t62 * t62;
        let t65 = piecewise3(t61, t24, t63 * t60);
        let t66 = t65 * t30;
        let t67 = param_bb * sigma2;
        let t68 = rho1 * rho1;
        let t69 = pow_1_3(rho1);
        let t70 = t69 * t69;
        let t72 = 1.0 / t70 / t68;
        let t75 = f64::sqrt(sigma2);
        let t76 = param_cc * t75;
        let t78 = 1.0 / t69 / rho1;
        let t81 = 4.0 * t75 * t78 + t44;
        let t82 = 1.0 / t81;
        let t83 = t78 * t82;
        let t85 = param_aa + 0.13888888888888888889e-1 * t67 * t72 + t76 * t83;
        let t89 = piecewise3(t56, 0.0, 3.0 / 20.0 * t6 * t66 * t85);
        let tzk0 = t55 + t89;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (54 lines) ---
        let t90 = t7 * t7;
        let t91 = 1.0 / t90;
        let t92 = t17 * t91;
        let t94 = piecewise5(t11, 0.0, t15, 0.0, t8 - t92);
        let t97 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t94);
        let t98 = t97 * t30;
        let t102 = 1.0 / t29;
        let t103 = t28 * t102;
        let t106 = t6 * t103 * t51 / 10.0;
        let t107 = t33 * rho0;
        let t109 = 1.0 / t35 / t107;
        let t114 = 1.0 / t34 / t33 * t48;
        let t117 = param_cc * sigma0;
        let t118 = t47 * t47;
        let t119 = 1.0 / t118;
        let t120 = t109 * t119;
        let t123 = -0.37037037037037037037e-1 * t32 * t109 - 4.0 / 3.0 * t41 * t114 + 16.0 / 3.0 * t117 * t120;
        let t128 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t98 * t51 + t106 + 3.0 / 20.0 * t6 * t31 * t123);
        let t129 = t57 * t91;
        let t131 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t129);
        let t134 = piecewise3(t61, 0.0, 5.0 / 3.0 * t63 * t131);
        let t135 = t134 * t30;
        let t139 = t65 * t102;
        let t142 = t6 * t139 * t85 / 10.0;
        let t144 = piecewise3(t56, 0.0, 3.0 / 20.0 * t6 * t135 * t85 + t142);
        let tvrho0 = t55 + t89 + t7 * (t128 + t144);
        vrho[ip * 2] += tvrho0;
        let t148 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t92);
        let t151 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t148);
        let t152 = t151 * t30;
        let t157 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t152 * t51 + t106);
        let t159 = piecewise5(t15, 0.0, t11, 0.0, t8 - t129);
        let t162 = piecewise3(t61, 0.0, 5.0 / 3.0 * t63 * t159);
        let t163 = t162 * t30;
        let t167 = t68 * rho1;
        let t169 = 1.0 / t70 / t167;
        let t174 = 1.0 / t69 / t68 * t82;
        let t177 = param_cc * sigma2;
        let t178 = t81 * t81;
        let t179 = 1.0 / t178;
        let t180 = t169 * t179;
        let t183 = -0.37037037037037037037e-1 * t67 * t169 - 4.0 / 3.0 * t76 * t174 + 16.0 / 3.0 * t177 * t180;
        let t188 = piecewise3(t56, 0.0, 3.0 / 20.0 * t6 * t163 * t85 + t142 + 3.0 / 20.0 * t6 * t66 * t183);
        let tvrho1 = t55 + t89 + t7 * (t157 + t188);
        vrho[ip * 2 + 1] += tvrho1;
        let t193 = 1.0 / t40;
        let t194 = param_cc * t193;
        let t200 = 0.13888888888888888889e-1 * param_bb * t37 + t194 * t49 / 2.0 - 2.0 * param_cc * t37 * t119;
        let t204 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t200);
        let tvsigma0 = t7 * t204;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t207 = 1.0 / t75;
        let t208 = param_cc * t207;
        let t214 = 0.13888888888888888889e-1 * param_bb * t72 + t208 * t83 / 2.0 - 2.0 * param_cc * t72 * t179;
        let t218 = piecewise3(t56, 0.0, 3.0 / 20.0 * t6 * t66 * t214);
        let tvsigma2 = t7 * t218;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
