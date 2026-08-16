//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1337/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1337<F: Float>(t17577: F, t129: F, t15844: F, t1558: F, t442: F, t79: F, t13439: F, t13452: F, t1972: F, t5176: F, t13502: F, t13504: F) -> (F, F, F, F, F, F, F, F) {
    let t17578 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t17577;
    let t17579 = t129 * t15844;
    let t17583 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17579 * t442 * t1558 * t79;
    let t17584 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13439;
    let t17585 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13452;
    let t17587 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1972 * t5176;
    let t17588 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13502;
    let t17589 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t13504;
    (t17578, t17579, t17583, t17584, t17585, t17587, t17588, t17589)
}
