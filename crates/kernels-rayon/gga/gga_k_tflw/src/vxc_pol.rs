//! GGA_K_TFLW vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_tflw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_tflw_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_lambda: f64,
    param_gamma: f64,
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
        let t32 = param_lambda * sigma0;
        let t33 = rho0 * rho0;
        let t34 = pow_1_3(rho0);
        let t35 = t34 * t34;
        let t37 = 1.0 / t35 / t33;
        let t38 = M_CBRT6;
        let t40 = M_PI * M_PI;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t47 = param_gamma + 5.0 / 72.0 * t32 * t37 * t38 * t43;
        let t51 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t47);
        let t52 = rho1 <= dens_threshold;
        let t53 = -t17;
        let t55 = piecewise5(t15, t12, t11, t16, t53 * t8);
        let t56 = 1.0 + t55;
        let t57 = t56 <= zeta_threshold;
        let t58 = pow_1_3(t56);
        let t59 = t58 * t58;
        let t61 = piecewise3(t57, t24, t59 * t56);
        let t62 = t61 * t30;
        let t63 = param_lambda * sigma2;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t73 = param_gamma + 5.0 / 72.0 * t63 * t68 * t38 * t43;
        let t77 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t62 * t73);
        let tzk0 = t51 + t77;
        zk[ip] += tzk0;
        let t78 = t7 * t7;
        let t79 = 1.0 / t78;
        let t80 = t17 * t79;
        let t82 = piecewise5(t11, 0.0, t15, 0.0, t8 - t80);
        let t85 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t82);
        let t86 = t85 * t30;
        let t90 = 1.0 / t29;
        let t91 = t28 * t90;
        let t94 = t6 * t91 * t47 / 10.0;
        let t95 = t6 * t31;
        let t98 = 1.0 / t35 / t33 / rho0;
        let t101 = t32 * t98 * t38 * t43;
        let t105 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t86 * t47 + t94 - t95 * t101 / 36.0);
        let t106 = t53 * t79;
        let t108 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t106);
        let t111 = piecewise3(t57, 0.0, 5.0 / 3.0 * t59 * t108);
        let t112 = t111 * t30;
        let t116 = t61 * t90;
        let t119 = t6 * t116 * t73 / 10.0;
        let t121 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t112 * t73 + t119);
        let tvrho0 = t51 + t77 + t7 * (t105 + t121);
        vrho[ip * 2] += tvrho0;
        let t125 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t80);
        let t128 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t125);
        let t129 = t128 * t30;
        let t134 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t129 * t47 + t94);
        let t136 = piecewise5(t15, 0.0, t11, 0.0, t8 - t106);
        let t139 = piecewise3(t57, 0.0, 5.0 / 3.0 * t59 * t136);
        let t140 = t139 * t30;
        let t144 = t6 * t62;
        let t147 = 1.0 / t66 / t64 / rho1;
        let t150 = t63 * t147 * t38 * t43;
        let t154 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t140 * t73 + t119 - t144 * t150 / 36.0);
        let tvrho1 = t51 + t77 + t7 * (t134 + t154);
        vrho[ip * 2 + 1] += tvrho1;
        let t158 = t38 * t43;
        let t159 = param_lambda * t37 * t158;
        let t162 = piecewise3(t1, 0.0, t95 * t159 / 96.0);
        let tvsigma0 = t7 * t162;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t164 = param_lambda * t68 * t158;
        let t167 = piecewise3(t52, 0.0, t144 * t164 / 96.0);
        let tvsigma2 = t7 * t167;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
