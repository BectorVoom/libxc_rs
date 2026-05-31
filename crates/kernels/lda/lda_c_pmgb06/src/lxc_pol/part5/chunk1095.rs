//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1095/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1095<F: Float>(t16491: F, t439: F, t6151: F, t12753: F, t20146: F, t20151: F, t20155: F, t20159: F, t20161: F, t20162: F, t20165: F, t20168: F, t20171: F, t20174: F) -> (F, F) {
    let t20177 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t439 * t16491 * t6151;
    let t20178 = t20146 + t20151 - t20155 + t20159 - t20161 - t20162 + t12753 - t20165 - t20168 + t20171 - t20174 + t20177;
    (t20177, t20178)
}
