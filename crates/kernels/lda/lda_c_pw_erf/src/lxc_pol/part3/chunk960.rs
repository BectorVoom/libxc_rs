//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 960/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk960<F: Float>(t9721: F, t9725: F, t9737: F, t9905: F, t493: F, t9946: F, t9909: F, t1508: F, t2134: F, t9923: F, t9925: F, t9928: F, t9931: F, t9934: F, t9936: F, t9939: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12740 = 16.0 / 27.0 * t9721;
    let t12741 = 8.0 / 27.0 * t9725;
    let t12742 = 32.0 / 45.0 * t9737;
    let t12743 = 8.0 / 15.0 * t9905;
    let t12745 = 4.0 / 5.0 * t493 * t9946;
    let t12746 = 16.0 / 135.0 * t9909;
    let t12747 = t1508 * t2134;
    let t12748 = 4.0 / 15.0 * t12747;
    let t12749 = 4.0 / 45.0 * t9923;
    let t12750 = 16.0 / 45.0 * t9925;
    let t12751 = 4.0 / 15.0 * t9928;
    let t12752 = 8.0 / 45.0 * t9931;
    let t12753 = 16.0 / 135.0 * t9934;
    let t12754 = 8.0 / 45.0 * t9936;
    let t12755 = t12740 - t12741 + t12742 - t12743 - t12745 - t12746 - t12748 + t12749 - t12750 - t12751 - t12752 - t12753 + t12754;
    let t12756 = 4.0 / 45.0 * t9939;
    (t12740, t12741, t12742, t12743, t12745, t12746, t12748, t12749, t12750, t12751, t12752, t12753, t12754, t12755, t12756)
}
