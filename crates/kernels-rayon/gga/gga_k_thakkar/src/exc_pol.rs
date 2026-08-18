//! GGA_K_THAKKAR exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_thakkar.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_thakkar_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t37 = sigma0 * t36;
        let t38 = f64::sqrt(sigma0);
        let t40 = 1.0 / t33 / rho0;
        let t41 = t38 * t40;
        let t42 = f64::ln(t41 + f64::sqrt(t41 * t41 + 1.0));
        let t45 = 1.0 + 0.0253 * t41 * t42;
        let t46 = 1.0 / t45;
        let t49 = M_CBRT4;
        let t50 = t49 * t38;
        let t53 = 2.0 * t50 * t40 + 1.0;
        let t54 = 1.0 / t53;
        let t57 = 1.0 + 0.0055 * t37 * t46 - 0.072 * t41 * t54;
        let t61 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t17;
        let t65 = piecewise5(t15, t12, t11, t16, t63 * t8);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3(t66);
        let t69 = t68 * t68;
        let t71 = piecewise3(t67, t24, t69 * t66);
        let t72 = t71 * t30;
        let t73 = rho1 * rho1;
        let t74 = pow_1_3(rho1);
        let t75 = t74 * t74;
        let t77 = 1.0 / t75 / t73;
        let t78 = sigma2 * t77;
        let t79 = f64::sqrt(sigma2);
        let t81 = 1.0 / t74 / rho1;
        let t82 = t79 * t81;
        let t83 = f64::ln(t82 + f64::sqrt(t82 * t82 + 1.0));
        let t86 = 1.0 + 0.0253 * t82 * t83;
        let t87 = 1.0 / t86;
        let t90 = t49 * t79;
        let t93 = 2.0 * t90 * t81 + 1.0;
        let t94 = 1.0 / t93;
        let t97 = 1.0 + 0.0055 * t78 * t87 - 0.072 * t82 * t94;
        let t101 = piecewise3(t62, 0.0, 3.0 / 20.0 * t6 * t72 * t97);
        let tzk0 = t61 + t101;
        zk[ip] += tzk0;
    }
}
