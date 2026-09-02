//! MGGA_X_GDME exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gdme.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gdme_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_AA: f64,
    param_BB: f64,
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
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
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
        let t31 = M_CBRT2;
        let t34 = pow_1_3(1.0 / M_PI);
        let t35 = 1.0 / t34;
        let t36 = M_CBRT4;
        let t37 = t35 * t36;
        let t38 = M_PI * M_PI;
        let t39 = pow_1_3(t38);
        let t40 = t39 * t39;
        let t44 = 2.0 / 9.0 * (param_AA + 3.0 / 5.0 * param_BB) * t31 * t37 / t40;
        let t46 = param_BB * t3 * t35;
        let t47 = t31 * t31;
        let t48 = t36 * t47;
        let t50 = 1.0 / t39 / t38;
        let t51 = param_a * param_a;
        let t52 = t51 - param_a + 1.0 / 2.0;
        let t53 = t52 * lapl0;
        let t54 = pow_1_3(rho0);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / rho0;
        let t66 = t44 + t46 * t48 * t50 * (t53 * t57 - 2.0 * t57 * tau0) / 27.0;
        let t70 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t17;
        let t74 = piecewise5(t15, t12, t11, t16, t72 * t8);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t23, t77 * t75);
        let t80 = t79 * t27;
        let t81 = t52 * lapl1;
        let t82 = pow_1_3(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / rho1;
        let t94 = t44 + t46 * t48 * t50 * (t81 * t85 - 2.0 * t85 * tau1) / 27.0;
        let t98 = piecewise3(t71, 0.0, -3.0 / 8.0 * t6 * t80 * t94);
        let tzk0 = t70 + t98;
        zk[ip] += tzk0;
    }
}
