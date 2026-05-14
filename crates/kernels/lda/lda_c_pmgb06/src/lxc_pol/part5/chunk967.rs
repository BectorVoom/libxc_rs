//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 967/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk967<F: Float>(t1925: F, t6134: F, t432: F, t7863: F, t161: F, t489: F, t7725: F, t16583: F, t531: F, t7628: F, t12829: F, t20182: F, t20186: F, t20189: F, t20191: F, t20194: F, t20197: F) -> (F, F, F, F, F, F) {
    let t20199 = t6134 * t1925 / 15.0;
    let t20201 = t432 * t7863 / 10.0;
    let t20203 = t161 * t489 * t7725;
    let t20204 = 2.0 / 15.0 * t20203;
    let t20205 = 2.0 / 15.0 * t16583;
    let t20207 = t7628 * t531 / 30.0;
    let t20208 = t20182 - t20186 - t20189 - t20191 - t20194 - t20197 - t20199 - t20201 + t20204 - t20205 - t20207 - t12829;
    (t20199, t20201, t20204, t20205, t20207, t20208)
}
