//! MGGA_X_EDMGGA exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_edmgga_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT4;
        let t30 = t3 * t3;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t35 = t29 * t30 * t33 / 9.0;
        let t36 = 1.0 - t35;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / rho0;
        let t42 = rho0 * rho0;
        let t44 = 1.0 / t38 / t42;
        let t50 = M_CBRT6;
        let t52 = t33 * t33;
        let t53 = 1.0 / t52;
        let t54 = (tau0 * t40 - sigma0 * t44 / 8.0 - lapl0 * t40 / 4.0) * t50 * t53;
        let t55 = 5.0 / 9.0 * t54;
        let t56 = -t55 < -14205.545454545454;
        let t57 = 0.39111111111111113 * t54;
        let t59 = 0.0 < 0.7041420454545455 - t57;
        let t61 = piecewise3(t59, -0.00014204545454545454, 0.704 - t57);
        let t64 = t61 * t61;
        let t65 = t64 * t61;
        let t66 = 1.0 / t65;
        let t69 = 1.0 - t55;
        let t70 = t69 * t69;
        let t72 = 1.0 + 0.495616 * t70;
        let t73 = rmath::sqrt(t72);
        let t75 = piecewise3(t56, -1.0 / t61 / 2.0 + t66 / 8.0, 0.704 - t57 + t73);
        let t76 = t36 * t75;
        let t77 = rmath::sqrt(30.0);
        let t78 = t36 * t77;
        let t79 = rmath::sqrt(t75);
        let t80 = t36 * t36;
        let t83 = 1.0 / t80 / t36 * t77;
        let t85 = 0.6018478308354863 * t80 - 0.0206514;
        let t86 = t75 - 1.0;
        let t90 = rmath::ln(0.3910293204892512 * t83 * t85 * t86 + rmath::sqrt(pow_2(0.3910293204892512 * t83 * t85 * t86) + 1.0));
        let t94 = 1.0 + 0.14163895778062927 * t78 * t79 * t90;
        let t95 = 1.0 / t94;
        let t97 = t76 * t95 + t35;
        let t101 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t97);
        let t102 = rho1 <= dens_threshold;
        let t103 = -t17;
        let t105 = piecewise5(t15, t12, t11, t16, t103 * t8);
        let t106 = 1.0 + t105;
        let t107 = t106 <= zeta_threshold;
        let t108 = pow_1_3(t106);
        let t110 = piecewise3(t107, t23, t108 * t106);
        let t111 = t110 * t27;
        let t112 = pow_1_3(rho1);
        let t113 = t112 * t112;
        let t115 = 1.0 / t113 / rho1;
        let t117 = rho1 * rho1;
        let t119 = 1.0 / t113 / t117;
        let t126 = (tau1 * t115 - sigma2 * t119 / 8.0 - lapl1 * t115 / 4.0) * t50 * t53;
        let t127 = 5.0 / 9.0 * t126;
        let t128 = -t127 < -14205.545454545454;
        let t129 = 0.39111111111111113 * t126;
        let t131 = 0.0 < 0.7041420454545455 - t129;
        let t133 = piecewise3(t131, -0.00014204545454545454, 0.704 - t129);
        let t136 = t133 * t133;
        let t137 = t136 * t133;
        let t138 = 1.0 / t137;
        let t141 = 1.0 - t127;
        let t142 = t141 * t141;
        let t144 = 1.0 + 0.495616 * t142;
        let t145 = rmath::sqrt(t144);
        let t147 = piecewise3(t128, -1.0 / t133 / 2.0 + t138 / 8.0, 0.704 - t129 + t145);
        let t148 = t36 * t147;
        let t149 = rmath::sqrt(t147);
        let t150 = t147 - 1.0;
        let t154 = rmath::ln(0.3910293204892512 * t83 * t85 * t150 + rmath::sqrt(pow_2(0.3910293204892512 * t83 * t85 * t150) + 1.0));
        let t158 = 1.0 + 0.14163895778062927 * t78 * t149 * t154;
        let t159 = 1.0 / t158;
        let t161 = t148 * t159 + t35;
        let t165 = piecewise3(t102, 0.0, -3.0 / 8.0 * t6 * t111 * t161);
        let tzk0 = t101 + t165;
        zk[ip] += tzk0;
    }
}
