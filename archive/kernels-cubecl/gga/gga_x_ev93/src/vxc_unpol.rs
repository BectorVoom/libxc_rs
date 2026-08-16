//! GGA_X_EV93 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ev93_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t6 * t17;
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = param_a1 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
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
        let t78 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t18 * t61 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
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
        let t120 = piecewise3::<f64>(t2, 0.0, -t18 * t80 * t74 / 8.0 - 3.0 / 8.0 * t18 * t102 * t74 + 3.0 / 8.0 * t18 * t61 * t115);
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
        let t150 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t18 * t135 * t74 + 3.0 / 8.0 * t18 * t61 * t145);
        let tvsigma0 = 2.0 * rho[ip] * t150;
        vsigma[ip] += tvsigma0;
    }
}
