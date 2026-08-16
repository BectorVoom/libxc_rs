//! MGGA_XC_CC06 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_cc06_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t9 = rho0 * t8;
        let t11 = 2.0 * t9 <= zeta_threshold;
        let t12 = pow_1_3::<f64>(zeta_threshold);
        let t13 = t12 * zeta_threshold;
        let t14 = M_CBRT2;
        let t15 = t14 * rho0;
        let t16 = pow_1_3::<f64>(t9);
        let t20 = piecewise3::<f64>(t11, t13, 2.0 * t15 * t8 * t16);
        let t21 = pow_1_3::<f64>(t7);
        let t25 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t20 * t21);
        let t26 = rho1 <= dens_threshold;
        let t27 = rho1 * t8;
        let t29 = 2.0 * t27 <= zeta_threshold;
        let t30 = t14 * rho1;
        let t31 = pow_1_3::<f64>(t27);
        let t35 = piecewise3::<f64>(t29, t13, 2.0 * t30 * t8 * t31);
        let t39 = piecewise3::<f64>(t26, 0.0, -3.0 / 8.0 * t6 * t35 * t21);
        let t40 = 1.0 / M_PI;
        let t41 = pow_1_3::<f64>(t40);
        let t42 = t3 * t41;
        let t43 = M_CBRT4;
        let t44 = t43 * t43;
        let t47 = t42 * t44 / t21;
        let t49 = 1.0 + 0.53425e-1 * t47;
        let t50 = f64::sqrt(t47);
        let t53 = pow_3_2::<f64>(t47);
        let t55 = t3 * t3;
        let t56 = t41 * t41;
        let t57 = t55 * t56;
        let t58 = t21 * t21;
        let t59 = 1.0 / t58;
        let t61 = t57 * t43 * t59;
        let t63 = 0.379785e1 * t50 + 0.8969e0 * t47 + 0.204775e0 * t53 + 0.123235e0 * t61;
        let t66 = 1.0 + 0.16081824322151104822e2 / t63;
        let t67 = f64::ln(t66);
        let t69 = 0.62182e-1 * t49 * t67;
        let t70 = rho0 - rho1;
        let t71 = t70 * t70;
        let t72 = t71 * t71;
        let t73 = t7 * t7;
        let t74 = t73 * t73;
        let t75 = 1.0 / t74;
        let t76 = t72 * t75;
        let t77 = t70 * t8;
        let t78 = 1.0 + t77;
        let t79 = t78 <= zeta_threshold;
        let t80 = pow_1_3::<f64>(t78);
        let t82 = piecewise3::<f64>(t79, t13, t80 * t78);
        let t83 = 1.0 - t77;
        let t84 = t83 <= zeta_threshold;
        let t85 = pow_1_3::<f64>(t83);
        let t87 = piecewise3::<f64>(t84, t13, t85 * t83);
        let t88 = t82 + t87 - 2.0;
        let t91 = 1.0 / (2.0 * t14 - 2.0);
        let t92 = t88 * t91;
        let t94 = 1.0 + 0.5137e-1 * t47;
        let t99 = 0.705945e1 * t50 + 0.1549425e1 * t47 + 0.420775e0 * t53 + 0.1562925e0 * t61;
        let t102 = 1.0 + 0.32164683177870697974e2 / t99;
        let t103 = f64::ln(t102);
        let t107 = 1.0 + 0.278125e-1 * t47;
        let t112 = 0.51785e1 * t50 + 0.905775e0 * t47 + 0.1100325e0 * t53 + 0.1241775e0 * t61;
        let t115 = 1.0 + 0.29608574643216675549e2 / t112;
        let t116 = f64::ln(t115);
        let t117 = t107 * t116;
        let t119 = -0.3109e-1 * t94 * t103 + t69 - 0.19751789702565206229e-1 * t117;
        let t120 = t92 * t119;
        let t124 = t25 + t39 - t69 + t76 * t120 + 0.19751789702565206229e-1 * t92 * t117;
        let t125 = t55 * t43;
        let t126 = pow_1_3::<f64>(rho0);
        let t127 = t126 * t126;
        let t129 = 1.0 / t127 / rho0;
        let t130 = lapl0 * t129;
        let t131 = t78 / 2.0;
        let t132 = pow_1_3::<f64>(t131);
        let t133 = t132 * t132;
        let t134 = t133 * t131;
        let t136 = pow_1_3::<f64>(rho1);
        let t137 = t136 * t136;
        let t139 = 1.0 / t137 / rho1;
        let t140 = lapl1 * t139;
        let t141 = t83 / 2.0;
        let t142 = pow_1_3::<f64>(t141);
        let t143 = t142 * t142;
        let t144 = t143 * t141;
        let t148 = t125 * t56 * (t130 * t134 + t140 * t144);
        let t150 = -0.7e-3 + 0.2e-2 * t148;
        let t152 = 1.0 + 0.65e-2 * t148;
        let t153 = 1.0 / t152;
        let t155 = t150 * t153 + 1.0;
        let tzk0 = t124 * t155;
        zk[ip] += tzk0;
    }
}
