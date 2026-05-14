//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 889/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk889<F: Float>(t518: F, t6208: F, t6579: F, t1450: F, t6988: F, t352: F, t743: F, t4738: F, t5310: F, t1318: F, t4794: F, t6370: F, t172: F, t184: F, t6629: F, t4561: F, t822: F) -> (F, F, F, F, F, F, F, F) {
    let t15685 = t6208 * t518;
    let t15694 = t6579 * t518;
    let t15697 = t6988 * t1450;
    let t15727 = t743 * t352;
    let t15743 = t4738 * t5310;
    let t15750 = t1318 * t4794 * t6370;
    let t15761 = t172 * t6629 * t184;
    let t15764 = t822 * t4561;
    (t15685, t15694, t15697, t15727, t15743, t15750, t15761, t15764)
}
