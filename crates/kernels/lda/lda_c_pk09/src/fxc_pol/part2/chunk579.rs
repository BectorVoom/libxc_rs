//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 579/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk579<F: Float>(t3130: F, t1106: F, t3290: F, t1098: F, t1101: F, t3230: F, t3233: F, t3522: F, t91: F, t114: F, t3163: F, t1063: F, t3498: F) -> (F, F, F, F, F, F, F, F) {
    let t4138 = F::cast_from(0.032891459774245305_f64) * t3130;
    let t4144 = t1106 * t3290 / F::new(6.0);
    let t4146 = t1098 * t3290 / F::new(6.0);
    let t4147 = t1101 * t3230;
    let t4149 = t1101 * t3233;
    let t4165 = t3522 * t91;
    let t4166 = t114 * t4165;
    let t4168 = t4166 * t3163 / F::new(3.0);
    let t4170 = F::new(2.0) / F::new(9.0) * t1063 * t3498;
    (t4138, t4144, t4146, t4147, t4149, t4165, t4168, t4170)
}
