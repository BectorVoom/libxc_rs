//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 339/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk339<F: Float>(t247: F, t290: F, t395: F, t701: F, t250: F) -> (F, F, F) {
    let t1205 = F::cast_from(0.31995040645307626_f64) * t247 * t290;
    let t1206 = t395 * t701;
    let t1212 = F::cast_from(1.0_f64) / t250;
    (t1205, t1206, t1212)
}
