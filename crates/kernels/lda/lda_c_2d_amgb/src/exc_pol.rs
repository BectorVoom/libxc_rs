//! LDA_C_2D_AMGB exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_amgb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_2D_AMGB exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_2d_amgb_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = f64::sqrt(t1);
        let t3 = 1.0 / t2;
        let t5 = 1.0 / t1;
        let t8 = 1.0 / t2 / t1;
        let t10 = 0.04869723403850762 * t3 + 0.018219548589342285 * t5 + 0.000603947002028882 * t8;
        let t12 = f64::sqrt(M_PI);
        let t13 = 1.0 / t12;
        let t14 = t13 * t3;
        let t15 = pow_3_2(t14);
        let t19 = 0.5654308006315614 * t3 - 0.02069 * t15 + 0.10821581200590331 * t5 + 0.00313738702352666 * t8;
        let t21 = 1.0 + 1.0 / t19;
        let t22 = f64::ln(t21);
        let t23 = t10 * t22;
        let t27 = -0.01914859446561085 * t3 - 0.0024406887987971425 * t5 - 1.643337945467037e-05 * t8;
        let t31 = 0.2331795548802877 * t3 + 0.021277965468762 * t5 + 0.0001400599965454174 * t8;
        let t33 = 1.0 + 1.0 / t31;
        let t34 = f64::ln(t33);
        let t36 = 0.117331 + t27 * t34;
        let t37 = rho0 - rho1;
        let t38 = t37 * t37;
        let t39 = t36 * t38;
        let t40 = t1 * t1;
        let t41 = 1.0 / t40;
        let t42 = t39 * t41;
        let t46 = -0.020927484222536923 * t3 + 0.005208122695761946 * t5 - 0.0048916627893863685 * t8;
        let t49 = 0.8035757880366529 * t3 + 0.2088776021566591 * t8;
        let t51 = 1.0 + 1.0 / t49;
        let t52 = f64::ln(t51);
        let t54 = 0.0234188 + t46 * t52;
        let t55 = t38 * t38;
        let t56 = t54 * t55;
        let t57 = t40 * t40;
        let t58 = 1.0 / t57;
        let t59 = t56 * t58;
        let t61 = f64::exp(-0.7552241765370266 * t3);
        let t63 = M_SQRT2;
        let t64 = (t61 - 1.0) * t63;
        let t65 = t13 * t2;
        let t66 = t37 * t5;
        let t67 = 1.0 + t66;
        let t68 = t67 <= zeta_threshold;
        let t69 = f64::sqrt(zeta_threshold);
        let t70 = t69 * zeta_threshold;
        let t71 = f64::sqrt(t67);
        let t72 = t71 * t67;
        let t73 = piecewise3(t68, t70, t72);
        let t75 = 1.0 - t66;
        let t76 = t75 <= zeta_threshold;
        let t77 = f64::sqrt(t75);
        let t78 = t77 * t75;
        let t79 = piecewise3(t76, t70, t78);
        let t85 = t73 / 2.0 + t79 / 2.0 - 1.0 - 3.0 / 8.0 * t38 * t41 - 3.0 / 128.0 * t55 * t58;
        let t88 = 4.0 / 3.0 * t64 * t65 * t85;
        let tzk0 = -0.1925 + t23 + t42 + t59 - t88;
        zk[ip] += tzk0;
    }
}
