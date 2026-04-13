//! GGA_X_MPBE vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_mpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_mpbe_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
    }
}
