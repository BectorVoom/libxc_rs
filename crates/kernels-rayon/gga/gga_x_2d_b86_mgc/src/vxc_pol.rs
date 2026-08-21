//! GGA_X_2D_B86_MGC vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86_mgc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b86_mgc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = rmath::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = rmath::sqrt(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = rmath::sqrt(t17);
        let t22 = t21 * t17;
        let t23 = piecewise3(t18, t20, t22);
        let t24 = t3 * t23;
        let t25 = M_SQRT2;
        let t26 = rmath::sqrt(t4);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = t28 * rho0;
        let t30 = 1.0 / t29;
        let t31 = sigma0 * t30;
        let t33 = 1.0 + 0.008323 * t31;
        let t34 = pow_1_4(t33);
        let t35 = t34 * t34;
        let t36 = t35 * t34;
        let t37 = 1.0 / t36;
        let t40 = 1.0 + 0.002204711033795099 * t31 * t37;
        let t41 = t27 * t40;
        let t44 = piecewise3(t1, 0.0, -2.0 / 3.0 * t24 * t41);
        let t45 = rho1 <= dens_threshold;
        let t46 = -t14;
        let t48 = piecewise5(t12, t9, t8, t13, t46 * t5);
        let t49 = 1.0 + t48;
        let t50 = t49 <= zeta_threshold;
        let t51 = rmath::sqrt(t49);
        let t52 = t51 * t49;
        let t53 = piecewise3(t50, t20, t52);
        let t54 = t3 * t53;
        let t55 = rho1 * rho1;
        let t56 = t55 * rho1;
        let t57 = 1.0 / t56;
        let t58 = sigma2 * t57;
        let t60 = 1.0 + 0.008323 * t58;
        let t61 = pow_1_4(t60);
        let t62 = t61 * t61;
        let t63 = t62 * t61;
        let t64 = 1.0 / t63;
        let t67 = 1.0 + 0.002204711033795099 * t58 * t64;
        let t68 = t27 * t67;
        let t71 = piecewise3(t45, 0.0, -2.0 / 3.0 * t54 * t68);
        let tzk0 = t44 + t71;
        zk[ip] += tzk0;
        let t72 = t4 * t4;
        let t73 = 1.0 / t72;
        let t74 = t14 * t73;
        let t76 = piecewise5(t8, 0.0, t12, 0.0, t5 - t74);
        let t79 = piecewise3(t18, 0.0, 3.0 / 2.0 * t21 * t76);
        let t80 = t3 * t79;
        let t84 = t25 / t26;
        let t85 = t84 * t40;
        let t87 = t24 * t85 / 3.0;
        let t88 = t28 * t28;
        let t89 = 1.0 / t88;
        let t93 = sigma0 * sigma0;
        let t94 = t88 * t29;
        let t95 = 1.0 / t94;
        let t98 = 1.0 / t36 / t33;
        let t101 = -0.006614133101385296 * sigma0 * t89 * t37 + 4.128707235212237e-05 * t93 * t95 * t98;
        let t102 = t27 * t101;
        let t106 = piecewise3(t1, 0.0, -2.0 / 3.0 * t80 * t41 - t87 - 2.0 / 3.0 * t24 * t102);
        let t107 = t46 * t73;
        let t109 = piecewise5(t12, 0.0, t8, 0.0, -t5 - t107);
        let t112 = piecewise3(t50, 0.0, 3.0 / 2.0 * t51 * t109);
        let t113 = t3 * t112;
        let t116 = t84 * t67;
        let t118 = t54 * t116 / 3.0;
        let t120 = piecewise3(t45, 0.0, -2.0 / 3.0 * t113 * t68 - t118);
        let tvrho0 = t44 + t71 + t4 * (t106 + t120);
        vrho[ip * 2] += tvrho0;
        let t124 = piecewise5(t8, 0.0, t12, 0.0, -t5 - t74);
        let t127 = piecewise3(t18, 0.0, 3.0 / 2.0 * t21 * t124);
        let t128 = t3 * t127;
        let t132 = piecewise3(t1, 0.0, -2.0 / 3.0 * t128 * t41 - t87);
        let t134 = piecewise5(t12, 0.0, t8, 0.0, t5 - t107);
        let t137 = piecewise3(t50, 0.0, 3.0 / 2.0 * t51 * t134);
        let t138 = t3 * t137;
        let t141 = t55 * t55;
        let t142 = 1.0 / t141;
        let t146 = sigma2 * sigma2;
        let t147 = t141 * t56;
        let t148 = 1.0 / t147;
        let t151 = 1.0 / t63 / t60;
        let t154 = -0.006614133101385296 * sigma2 * t142 * t64 + 4.128707235212237e-05 * t146 * t148 * t151;
        let t155 = t27 * t154;
        let t159 = piecewise3(t45, 0.0, -2.0 / 3.0 * t138 * t68 - t118 - 2.0 / 3.0 * t54 * t155);
        let tvrho1 = t44 + t71 + t4 * (t132 + t159);
        vrho[ip * 2 + 1] += tvrho1;
        let t164 = t88 * t28;
        let t165 = 1.0 / t164;
        let t166 = sigma0 * t165;
        let t169 = 0.002204711033795099 * t30 * t37 - 1.3762357450707456e-05 * t166 * t98;
        let t170 = t27 * t169;
        let t173 = piecewise3(t1, 0.0, -2.0 / 3.0 * t24 * t170);
        let tvsigma0 = t4 * t173;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t176 = t141 * t55;
        let t177 = 1.0 / t176;
        let t178 = sigma2 * t177;
        let t181 = 0.002204711033795099 * t57 * t64 - 1.3762357450707456e-05 * t178 * t151;
        let t182 = t27 * t181;
        let t185 = piecewise3(t45, 0.0, -2.0 / 3.0 * t54 * t182);
        let tvsigma2 = t4 * t185;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
