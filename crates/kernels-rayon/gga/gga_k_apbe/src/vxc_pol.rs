//! GGA_K_APBE vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_apbe_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
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
        let t32 = M_CBRT6;
        let t33 = param_mu * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t47 = param_kappa + t33 * t38 * t43 / 24.0;
        let t52 = 1.0 + param_kappa * (1.0 - param_kappa / t47);
        let t56 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t17;
        let t60 = piecewise5(t15, t12, t11, t16, t58 * t8);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t64 = t63 * t63;
        let t66 = piecewise3(t62, t24, t64 * t61);
        let t67 = t66 * t30;
        let t68 = t37 * sigma2;
        let t69 = rho1 * rho1;
        let t70 = pow_1_3(rho1);
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t69;
        let t77 = param_kappa + t33 * t68 * t73 / 24.0;
        let t82 = 1.0 + param_kappa * (1.0 - param_kappa / t77);
        let t86 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t67 * t82);
        let tzk0 = t56 + t86;
        zk[ip] += tzk0;
        let t87 = t7 * t7;
        let t88 = 1.0 / t87;
        let t89 = t17 * t88;
        let t91 = piecewise5(t11, 0.0, t15, 0.0, t8 - t89);
        let t94 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t91);
        let t95 = t94 * t30;
        let t99 = 1.0 / t29;
        let t100 = t28 * t99;
        let t103 = t6 * t100 * t52 / 10.0;
        let t104 = param_kappa * param_kappa;
        let t105 = t31 * t104;
        let t106 = t6 * t105;
        let t107 = t47 * t47;
        let t109 = 1.0 / t107 * param_mu;
        let t110 = t109 * t32;
        let t111 = t39 * rho0;
        let t113 = 1.0 / t41 / t111;
        let t115 = t110 * t38 * t113;
        let t119 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t95 * t52 + t103 - t106 * t115 / 60.0);
        let t120 = t58 * t88;
        let t122 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t120);
        let t125 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t122);
        let t126 = t125 * t30;
        let t130 = t66 * t99;
        let t133 = t6 * t130 * t82 / 10.0;
        let t135 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t126 * t82 + t133);
        let tvrho0 = t56 + t86 + t7 * (t119 + t135);
        vrho[ip * 2] += tvrho0;
        let t139 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t89);
        let t142 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t139);
        let t143 = t142 * t30;
        let t148 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t143 * t52 + t103);
        let t150 = piecewise5(t15, 0.0, t11, 0.0, t8 - t120);
        let t153 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t150);
        let t154 = t153 * t30;
        let t158 = t67 * t104;
        let t159 = t6 * t158;
        let t160 = t77 * t77;
        let t162 = 1.0 / t160 * param_mu;
        let t163 = t162 * t32;
        let t164 = t69 * rho1;
        let t166 = 1.0 / t71 / t164;
        let t168 = t163 * t68 * t166;
        let t172 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t154 * t82 + t133 - t159 * t168 / 60.0);
        let tvrho1 = t56 + t86 + t7 * (t148 + t172);
        vrho[ip * 2 + 1] += tvrho1;
        let t175 = t32 * t37;
        let t177 = t109 * t175 * t43;
        let t180 = piecewise3(t1, 0.0, t106 * t177 / 160.0);
        let tvsigma0 = t7 * t180;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t182 = t162 * t175 * t73;
        let t185 = piecewise3(t57, 0.0, t159 * t182 / 160.0);
        let tvsigma2 = t7 * t185;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
