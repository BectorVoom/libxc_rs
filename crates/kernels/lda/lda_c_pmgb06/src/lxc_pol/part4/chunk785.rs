//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 785/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk785<F: Float>(t2012: F, t5211: F, t3226: F, t835: F, t1447: F, t1977: F, t1423: F, t1963: F, t607: F, t801: F) -> (F, F, F, F, F, F) {
    let t5212 = t5211 * t2012;
    let t5213 = F::new(2.0) / F::new(27.0) * t5212;
    let t5215 = F::new(4.0) / F::new(135.0) * t3226 * t835;
    let t5217 = F::new(4.0) / F::new(135.0) * t1447 * t1977;
    let t5219 = F::new(4.0) / F::new(135.0) * t1423 * t1963;
    let t5220 = t801 * t607;
    (t5212, t5213, t5215, t5217, t5219, t5220)
}
