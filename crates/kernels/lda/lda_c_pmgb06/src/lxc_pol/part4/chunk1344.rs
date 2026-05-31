//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1344/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1344<F: Float>(t4817: F, t802: F, t165: F, t1835: F, t1994: F, t493: F, t13706: F, t439: F, t5202: F, t6550: F, t1423: F, t6259: F) -> (F, F, F, F, F) {
    let t17657 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t802 * t4817;
    let t17661 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t493 * t165 * t1835 * t1994;
    let t17662 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13706;
    let t17665 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t439 * t6550 * t5202;
    let t17666 = t1423 * t6259;
    (t17657, t17661, t17662, t17665, t17666)
}
