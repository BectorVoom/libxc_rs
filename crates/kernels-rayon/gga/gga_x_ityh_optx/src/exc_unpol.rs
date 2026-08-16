//! GGA_X_ITYH_OPTX exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_optx.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_optx_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = sigma[ip] * sigma[ip];
        let t28 = param_b * t27;
        let t29 = M_CBRT2;
        let t30 = rho[ip] * rho[ip];
        let t31 = t30 * t30;
        let t32 = t31 * rho[ip];
        let t34 = 1.0 / t19 / t32;
        let t36 = t29 * t29;
        let t38 = t19 * t19;
        let t40 = 1.0 / t38 / t30;
        let t43 = 1.0 + 6.0 * sigma[ip] * t36 * t40;
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t29 * t34 * t45;
        let t49 = param_a + 72.0 * t28 * t46;
        let t52 = M_PI * t20 * t26 / t49;
        let t53 = f64::sqrt(t52);
        let t55 = param_hyb_omega_0 / t53;
        let t56 = t11 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t59 = t29 * t58;
        let t61 = t55 * t59 / 2.0;
        let t62 = 0.135e1 <= t61;
        let t63 = 0.135e1 < t61;
        let t64 = piecewise3(t63, t61, 0.135e1);
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
        let t90 = piecewise3(t63, 0.135e1, t61);
        let t91 = f64::sqrt(M_PI);
        let t92 = 1.0 / t90;
        let t94 = erf_approx(t92 / 2.0);
        let t96 = t90 * t90;
        let t97 = 1.0 / t96;
        let t99 = f64::exp(-t97 / 4.0);
        let t100 = t99 - 1.0;
        let t103 = t99 - 3.0 / 2.0 - 2.0 * t96 * t100;
        let t106 = 2.0 * t90 * t103 + t91 * t94;
        let t110 = piecewise3(t62, 1.0 / t65 / 36.0 - t69 / 960.0 + t72 / 26880.0 - t75 / 829440.0 + t78 / 28385280.0 - t81 / 0.107347968e10 + t84 / 0.445906944e11 - t87 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t90 * t106);
        let t111 = t19 * t110;
        let t115 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t111 * t49);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
    }
}
