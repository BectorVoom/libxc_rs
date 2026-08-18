//! GGA_X_ITYH_PBE exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_pbe_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t18 = t6 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t21 = M_PI * t20;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = M_CBRT6;
        let t28 = param_mu * t27;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t34 = M_CBRT2;
        let t35 = t34 * t34;
        let t36 = sigma[ip] * t35;
        let t37 = rho[ip] * rho[ip];
        let t38 = t19 * t19;
        let t40 = 1.0 / t38 / t37;
        let t44 = param_kappa + t28 * t32 * t36 * t40 / 24.0;
        let t49 = 1.0 + param_kappa * (1.0 - param_kappa / t44);
        let t52 = t21 * t26 / t49;
        let t53 = f64::sqrt(t52);
        let t55 = param_hyb_omega_0 / t53;
        let t56 = t11 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t61 = t55 * t34 * t58 / 2.0;
        let t62 = 1.35 <= t61;
        let t63 = 1.35 < t61;
        let t64 = piecewise3(t63, t61, 1.35);
        let t65 = t64 * t64;
        let t68 = t65 * t65;
        let t69 = 1.0 / t68;
        let t71 = t68 * t65;
        let t72 = 1.0 / t71;
        let t74 = t68 * t68;
        let t75 = 1.0 / t74;
        let t78 = 1.0 / t74 / t65;
        let t81 = 1.0 / t74 / t68;
        let t84 = 1.0 / t74 / t71;
        let t86 = t74 * t74;
        let t87 = 1.0 / t86;
        let t90 = piecewise3(t63, 1.35, t61);
        let t91 = f64::sqrt(M_PI);
        let t92 = 1.0 / t90;
        let t94 = erf_approx(t92 / 2.0);
        let t96 = t90 * t90;
        let t97 = 1.0 / t96;
        let t99 = f64::exp(-t97 / 4.0);
        let t100 = t99 - 1.0;
        let t103 = t99 - 3.0 / 2.0 - 2.0 * t96 * t100;
        let t106 = 2.0 * t90 * t103 + t91 * t94;
        let t110 = piecewise3(t62, 1.0 / t65 / 36.0 - t69 / 960.0 + t72 / 26880.0 - t75 / 829440.0 + t78 / 28385280.0 - t81 / 1073479680.0 + t84 / 44590694400.0 - t87 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t90 * t106);
        let t115 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t110 * t49);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
    }
}
