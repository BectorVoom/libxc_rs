//! LDA_C_CHACHIYO vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_chachiyo_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = rmath::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = rmath::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3(zeta_threshold);
        let t42 = t41 * zeta_threshold;
        let t43 = pow_1_3(t39);
        let t45 = piecewise3(t40, t42, t43 * t39);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3(t46);
        let t50 = piecewise3(t47, t42, t48 * t46);
        let t51 = t45 + t50 - 2.0;
        let t53 = M_CBRT2;
        let t56 = 1.0 / (2.0 * t53 - 2.0);
        let t57 = t35 * t51 * t56;
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
        let t80 = t78 * t51 * t56;
        let t81 = t9 * t9;
        let t82 = 1.0 / t81;
        let t83 = t36 * t82;
        let t84 = t37 - t83;
        let t87 = piecewise3(t40, 0.0, 4.0 / 3.0 * t43 * t84);
        let t88 = -t84;
        let t91 = piecewise3(t47, 0.0, 4.0 / 3.0 * t48 * t88);
        let t92 = t87 + t91;
        let t94 = t35 * t92 * t56;
        let tvrho0 = t25 + t57 + t9 * (t69 + t80 + t94);
        vrho[ip * 2] += tvrho0;
        let t97 = -t37 - t83;
        let t100 = piecewise3(t40, 0.0, 4.0 / 3.0 * t43 * t97);
        let t101 = -t97;
        let t104 = piecewise3(t47, 0.0, 4.0 / 3.0 * t48 * t101);
        let t105 = t100 + t104;
        let t107 = t35 * t105 * t56;
        let tvrho1 = t25 + t57 + t9 * (t69 + t80 + t107);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
