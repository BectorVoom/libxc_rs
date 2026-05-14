//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1165/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1165<F: Float>(t13439: F, t13452: F, t1972: F, t5176: F, t13502: F, t13504: F, t13507: F, t10720: F, t10727: F, t10735: F, t17564: F, t17571: F, t17575: F, t17576: F, t17578: F, t17583: F) -> (F, F, F, F, F, F, F) {
    let t17584 = 4.0 / 135.0 * t13439;
    let t17585 = 4.0 / 45.0 * t13452;
    let t17587 = 4.0 / 15.0 * t1972 * t5176;
    let t17588 = 8.0 / 135.0 * t13502;
    let t17589 = 4.0 / 135.0 * t13504;
    let t17590 = 4.0 / 81.0 * t13507;
    let t17591 = -t17564 + 8.0 / 3.0 * t10720 + t10727 + 4.0 / 3.0 * t10735 + t17571 - t17575 + t17576 - t17578 - t17583 + t17584 + t17585 + t17587 + t17588 + t17589 + t17590;
    (t17584, t17585, t17587, t17588, t17589, t17590, t17591)
}
