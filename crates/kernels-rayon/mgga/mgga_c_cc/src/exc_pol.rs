//! MGGA_C_CC exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cc_exc_pol(
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
        let t3 = sigma0 + 2.0 * sigma1 + sigma2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(t4);
        let t8 = t7 * t7;
        let t10 = 1.0 / t8 / t6;
        let t11 = t3 * t10;
        let t12 = pow_1_3(rho0);
        let t13 = t12 * t12;
        let t15 = 1.0 / t13 / rho0;
        let t16 = tau0 * t15;
        let t17 = rho0 - rho1;
        let t18 = 1.0 / t4;
        let t19 = t17 * t18;
        let t20 = 1.0 + t19;
        let t21 = t20 / 2.0;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = t23 * t21;
        let t26 = pow_1_3(rho1);
        let t27 = t26 * t26;
        let t29 = 1.0 / t27 / rho1;
        let t30 = tau1 * t29;
        let t31 = 1.0 - t19;
        let t32 = t31 / 2.0;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = t34 * t32;
        let t37 = t16 * t24 + t30 * t35;
        let t38 = 1.0 / t37;
        let t39 = t17 * t17;
        let t40 = t38 * t39;
        let t43 = 1.0 - t11 * t40 / 8.0;
        let t44 = M_CBRT3;
        let t45 = 1.0 / M_PI;
        let t46 = pow_1_3(t45);
        let t47 = t44 * t46;
        let t48 = M_CBRT4;
        let t49 = t48 * t48;
        let t52 = t47 * t49 / t7;
        let t54 = 1.0 + 0.53425e-1 * t52;
        let t55 = f64::sqrt(t52);
        let t58 = pow_3_2(t52);
        let t60 = t44 * t44;
        let t61 = t46 * t46;
        let t62 = t60 * t61;
        let t65 = t62 * t48 / t8;
        let t67 = 0.379785e1 * t55 + 0.8969e0 * t52 + 0.204775e0 * t58 + 0.123235e0 * t65;
        let t70 = 1.0 + 0.16081979498692535067e2 / t67;
        let t71 = f64::ln(t70);
        let t73 = 0.621814e-1 * t54 * t71;
        let t74 = t39 * t39;
        let t75 = 1.0 / t6;
        let t76 = t74 * t75;
        let t77 = t20 <= zeta_threshold;
        let t78 = pow_1_3(zeta_threshold);
        let t79 = t78 * zeta_threshold;
        let t80 = pow_1_3(t20);
        let t82 = piecewise3(t77, t79, t80 * t20);
        let t83 = t31 <= zeta_threshold;
        let t84 = pow_1_3(t31);
        let t86 = piecewise3(t83, t79, t84 * t31);
        let t87 = t82 + t86 - 2.0;
        let t88 = M_CBRT2;
        let t91 = 1.0 / (2.0 * t88 - 2.0);
        let t92 = t87 * t91;
        let t94 = 1.0 + 0.5137e-1 * t52;
        let t99 = 0.705945e1 * t55 + 0.1549425e1 * t52 + 0.420775e0 * t58 + 0.1562925e0 * t65;
        let t102 = 1.0 + 0.32163958997385070134e2 / t99;
        let t103 = f64::ln(t102);
        let t107 = 1.0 + 0.278125e-1 * t52;
        let t112 = 0.51785e1 * t55 + 0.905775e0 * t52 + 0.1100325e0 * t58 + 0.1241775e0 * t65;
        let t115 = 1.0 + 0.29608749977793437516e2 / t112;
        let t116 = f64::ln(t115);
        let t117 = t107 * t116;
        let t119 = -0.310907e-1 * t94 * t103 + t73 - 0.19751673498613801407e-1 * t117;
        let t120 = t92 * t119;
        let t124 = -t73 + t76 * t120 + 0.19751673498613801407e-1 * t92 * t117;
        let tzk0 = t43 * t124;
        zk[ip] += tzk0;
    }
}
