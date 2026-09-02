//! MGGA_K_CSK_LOC exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_csk_loc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_csk_loc_exc_unpol(
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
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t39 = 5.0 / 72.0 * t30 * t37;
        let t40 = param_csk_cp * t25;
        let t41 = t40 * t29;
        let t44 = param_csk_cq * t25;
        let t45 = t44 * t29;
        let t46 = lapl[ip] * t32;
        let t48 = 1.0 / t23 / rho[ip];
        let t52 = t41 * t37 / 24.0 + t45 * t46 * t48 / 24.0 - t39;
        let t54 = rmath::ln(1.0 - f64::EPSILON);
        let t55 = 1.0 / param_csk_a;
        let t56 = rmath::pow(-t54, -t55);
        let t57 = t52 < -t56;
        let t58 = rmath::ln(f64::EPSILON);
        let t59 = rmath::pow(-t58, -t55);
        let t60 = -t59 < t52;
        let t61 = piecewise3(t60, -t59, t52);
        let t62 = -t56 < t61;
        let t63 = piecewise3(t62, t61, -t56);
        let t64 = rmath::abs(t63);
        let t65 = rmath::pow(t64, param_csk_a);
        let t66 = 1.0 / t65;
        let t67 = rmath::exp(-t66);
        let t68 = 1.0 - t67;
        let t69 = rmath::pow(t68, t55);
        let t70 = piecewise5(t57, 0.0, t60, 1.0, t69);
        let t72 = t52 * t70 + t39 + 1.0;
        let t76 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
    }
}
