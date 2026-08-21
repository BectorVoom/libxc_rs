//! MGGA_K_CSK_LOC exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_csk_loc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_csk_loc_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_csk_cp: f64,
    param_csk_cq: f64,
    param_csk_a: f64,
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t46 = 5.0 / 72.0 * t38 * sigma0 * t43;
        let t47 = param_csk_cp * t33;
        let t48 = t37 * sigma0;
        let t52 = param_csk_cq * t33;
        let t53 = t37 * lapl0;
        let t55 = 1.0 / t41 / rho0;
        let t59 = t47 * t48 * t43 / 24.0 + t52 * t53 * t55 / 24.0 - t46;
        let t61 = rmath::ln(1.0 - f64::EPSILON);
        let t62 = 1.0 / param_csk_a;
        let t63 = rmath::pow(-t61, -t62);
        let t64 = t59 < -t63;
        let t65 = rmath::ln(f64::EPSILON);
        let t66 = rmath::pow(-t65, -t62);
        let t67 = -t66 < t59;
        let t68 = piecewise3(t67, -t66, t59);
        let t69 = -t63 < t68;
        let t70 = piecewise3(t69, t68, -t63);
        let t71 = rmath::abs(t70);
        let t72 = rmath::pow(t71, param_csk_a);
        let t73 = 1.0 / t72;
        let t74 = rmath::exp(-t73);
        let t75 = 1.0 - t74;
        let t76 = rmath::pow(t75, t62);
        let t77 = piecewise5(t64, 0.0, t67, 1.0, t76);
        let t79 = t59 * t77 + t46 + 1.0;
        let t83 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t79);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t18;
        let t87 = piecewise5(t16, t13, t12, t17, t85 * t9);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3(t88);
        let t91 = t90 * t90;
        let t93 = piecewise3(t89, t25, t91 * t88);
        let t94 = t93 * t31;
        let t95 = rho1 * rho1;
        let t96 = pow_1_3(rho1);
        let t97 = t96 * t96;
        let t99 = 1.0 / t97 / t95;
        let t102 = 5.0 / 72.0 * t38 * sigma2 * t99;
        let t103 = t37 * sigma2;
        let t107 = t37 * lapl1;
        let t109 = 1.0 / t97 / rho1;
        let t113 = t47 * t103 * t99 / 24.0 + t52 * t107 * t109 / 24.0 - t102;
        let t114 = t113 < -t63;
        let t115 = -t66 < t113;
        let t116 = piecewise3(t115, -t66, t113);
        let t117 = -t63 < t116;
        let t118 = piecewise3(t117, t116, -t63);
        let t119 = rmath::abs(t118);
        let t120 = rmath::pow(t119, param_csk_a);
        let t121 = 1.0 / t120;
        let t122 = rmath::exp(-t121);
        let t123 = 1.0 - t122;
        let t124 = rmath::pow(t123, t62);
        let t125 = piecewise5(t114, 0.0, t115, 1.0, t124);
        let t127 = t113 * t125 + t102 + 1.0;
        let t131 = piecewise3(t84, 0.0, 3.0 / 20.0 * t7 * t94 * t127);
        let tzk0 = t83 + t131;
        zk[ip] += tzk0;
    }
}
