//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 871/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk871<F: Float>(t1098: F, t9049: F, t1063: F, t4368: F, t7589: F, t120: F, t902: F, t7577: F, t1076: F, t1095: F, t1101: F, t2355: F, t3138: F, t3195: F, t3201: F, t4275: F, t4278: F, t8892: F, t8895: F, t9037: F, t9040: F, t9043: F, t9046: F, t98: F) -> (F, F, F) {
    let t9050 = t1098 * t9049;
    let t9054 = t1063 * t9049;
    let t9056 = t4368 * t7589;
    let t9059 = t120 * t902;
    let t9060 = t9059 * t7577;
    let t9069 = F::cast_from(0.14975624337724558_f64) * t3195 + F::cast_from(0.14975624337724558_f64) * t3201 - t9037 * t98 / F::new(6.0) + t9040 / F::new(6.0) + t1101 * t9043 / F::new(3.0) + t1101 * t9046 / F::new(6.0) + t9050 / F::new(6.0) + t1095 * t8892 / F::new(6.0) - t9054 / F::new(6.0) - t1076 * t9056 / F::new(6.0) - t1076 * t9060 / F::new(3.0) + t1095 * t8895 / F::new(6.0) + t2355 * t3138 / F::new(6.0) + t4275 / F::new(9.0) - t4278 / F::new(54.0);
    (t9056, t9060, t9069)
}
