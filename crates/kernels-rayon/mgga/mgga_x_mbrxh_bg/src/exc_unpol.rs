//! MGGA_X_MBRXH_BG exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbrxh_bg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbrxh_bg_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = tau[ip] * t23;
        let t25 = t15 * t15;
        let t27 = 1.0 / t25 / rho[ip];
        let t30 = M_CBRT6;
        let t31 = t30 * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t37 = sigma[ip] * t23;
        let t38 = rho[ip] * rho[ip];
        let t40 = 1.0 / t25 / t38;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t22;
        let t45 = t38 * t38;
        let t46 = t45 * rho[ip];
        let t48 = 1.0 / t15 / t46;
        let t51 = 0.46864 * t24 * t27 - 3.0 / 10.0 * t31 * t34 + 0.089 * t37 * t40 + 0.0106 * t44 * t48;
        let t52 = f64::abs(t51);
        let t53 = t52 < 5e-13;
        let t54 = 0.0 < t51;
        let t55 = piecewise3(t54, 5e-13, -5e-13);
        let t56 = piecewise3(t53, t55, t51);
        let t57 = xc_mgga_x_br89_get_x(t56);
        let t59 = f64::exp(t57 / 3.0);
        let t60 = t21 * t59;
        let t61 = f64::exp(-t57);
        let t63 = 1.0 + t57 / 2.0;
        let t64 = t61 * t63;
        let t65 = 1.0 - t64;
        let t66 = 1.0 / t57;
        let t67 = t65 * t66;
        let t68 = t60 * t67;
        let t71 = piecewise3(t3, 0.0, -t20 * t68 / 4.0);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
    }
}
