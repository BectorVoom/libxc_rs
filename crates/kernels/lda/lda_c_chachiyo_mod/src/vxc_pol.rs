//! LDA_C_CHACHIYO_MOD vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo_mod.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_CHACHIYO_MOD vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_chachiyo_mod_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3::<f64>(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3::<f64>(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = f64::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = f64::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3::<f64>(zeta_threshold);
        let t42 = t41 * t41;
        let t43 = pow_1_3::<f64>(t39);
        let t44 = t43 * t43;
        let t45 = piecewise3::<f64>(t40, t42, t44);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3::<f64>(t46);
        let t49 = t48 * t48;
        let t50 = piecewise3::<f64>(t47, t42, t49);
        let t52 = t45 / 2.0 + t50 / 2.0;
        let t53 = t52 * t52;
        let t56 = -2.0 * t53 * t52 + 2.0;
        let t57 = t35 * t56;
        let tzk0 = t25 + t57;
        zk[ip] += tzk0;
        let t59 = t8 / t19;
        let t63 = t18 / t10;
        let t66 = t3 * t59 / 9.0 + 2.0 / 9.0 * t14 * t63;
        let t68 = 1.0 / t23;
        let t69 = param_ap * t66 * t68;
        let t74 = t26 * t59 / 9.0 + 2.0 / 9.0 * t29 * t63;
        let t76 = 1.0 / t32;
        let t78 = param_af * t74 * t76 - t69;
        let t79 = t78 * t56;
        let t80 = t35 * t53;
        let t81 = 1.0 / t43;
        let t82 = t9 * t9;
        let t83 = 1.0 / t82;
        let t84 = t36 * t83;
        let t85 = t37 - t84;
        let t88 = piecewise3::<f64>(t40, 0.0, 2.0 / 3.0 * t81 * t85);
        let t89 = 1.0 / t48;
        let t90 = -t85;
        let t93 = piecewise3::<f64>(t47, 0.0, 2.0 / 3.0 * t89 * t90);
        let t95 = t88 / 2.0 + t93 / 2.0;
        let t96 = t80 * t95;
        let t97 = 6.0 * t96;
        let tvrho0 = t25 + t57 + t9 * (t69 + t79 - t97);
        vrho[ip * 2] += tvrho0;
        let t100 = -t37 - t84;
        let t103 = piecewise3::<f64>(t40, 0.0, 2.0 / 3.0 * t81 * t100);
        let t104 = -t100;
        let t107 = piecewise3::<f64>(t47, 0.0, 2.0 / 3.0 * t89 * t104);
        let t109 = t103 / 2.0 + t107 / 2.0;
        let t110 = t80 * t109;
        let t111 = 6.0 * t110;
        let tvrho1 = t25 + t57 + t9 * (t69 + t79 - t111);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
