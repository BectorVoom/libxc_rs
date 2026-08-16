//! MGGA_C_LTAPW exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ltapw.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ltapw_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_ltafrac: f64,
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
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho0);
        let t9 = t8 * t8;
        let t13 = M_CBRT6;
        let t14 = M_PI * M_PI;
        let t15 = pow_1_3(t14);
        let t16 = t15 * t15;
        let t18 = t13 / t16;
        let t21 = 3.0 / 5.0 * param_ltafrac;
        let t22 = f64::powf(5.0 / 9.0 * tau0 / t9 / rho0 * t18, t21);
        let t23 = rho0 * t22;
        let t24 = pow_1_3(rho1);
        let t25 = t24 * t24;
        let t31 = f64::powf(5.0 / 9.0 * tau1 / t25 / rho1 * t18, t21);
        let t32 = rho1 * t31;
        let t33 = t23 + t32;
        let t34 = pow_1_3(t33);
        let t37 = t5 * t7 / t34;
        let t39 = 1.0 + 0.53425e-1 * t37;
        let t40 = f64::sqrt(t37);
        let t43 = pow_3_2(t37);
        let t45 = t2 * t2;
        let t46 = t4 * t4;
        let t47 = t45 * t46;
        let t48 = t34 * t34;
        let t51 = t47 * t6 / t48;
        let t53 = 0.379785e1 * t40 + 0.8969e0 * t37 + 0.204775e0 * t43 + 0.123235e0 * t51;
        let t56 = 1.0 + 0.16081824322151104822e2 / t53;
        let t57 = f64::ln(t56);
        let t59 = 0.62182e-1 * t39 * t57;
        let t60 = t23 - t32;
        let t61 = t60 * t60;
        let t62 = t61 * t61;
        let t63 = t33 * t33;
        let t64 = t63 * t63;
        let t65 = 1.0 / t64;
        let t66 = t62 * t65;
        let t67 = 1.0 / t33;
        let t68 = t60 * t67;
        let t69 = 1.0 + t68;
        let t70 = t69 <= zeta_threshold;
        let t71 = pow_1_3(zeta_threshold);
        let t72 = t71 * zeta_threshold;
        let t73 = pow_1_3(t69);
        let t75 = piecewise3(t70, t72, t73 * t69);
        let t76 = 1.0 - t68;
        let t77 = t76 <= zeta_threshold;
        let t78 = pow_1_3(t76);
        let t80 = piecewise3(t77, t72, t78 * t76);
        let t81 = t75 + t80 - 2.0;
        let t82 = M_CBRT2;
        let t85 = 1.0 / (2.0 * t82 - 2.0);
        let t86 = t81 * t85;
        let t88 = 1.0 + 0.5137e-1 * t37;
        let t93 = 0.705945e1 * t40 + 0.1549425e1 * t37 + 0.420775e0 * t43 + 0.1562925e0 * t51;
        let t96 = 1.0 + 0.32164683177870697974e2 / t93;
        let t97 = f64::ln(t96);
        let t101 = 1.0 + 0.278125e-1 * t37;
        let t106 = 0.51785e1 * t40 + 0.905775e0 * t37 + 0.1100325e0 * t43 + 0.1241775e0 * t51;
        let t109 = 1.0 + 0.29608574643216675549e2 / t106;
        let t110 = f64::ln(t109);
        let t111 = t101 * t110;
        let t113 = -0.3109e-1 * t88 * t97 + t59 - 0.19751789702565206229e-1 * t111;
        let t114 = t86 * t113;
        let t115 = t66 * t114;
        let t117 = 0.19751789702565206229e-1 * t86 * t111;
        let tzk0 = -t59 + t115 + t117;
        zk[ip] += tzk0;
    }
}
