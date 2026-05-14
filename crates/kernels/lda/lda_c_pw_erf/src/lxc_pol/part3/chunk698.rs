//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 698/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk698<F: Float>(t1446: F, t1987: F, t1326: F, t4637: F, t519: F, t1991: F, t4615: F, t4633: F, t4829: F, t1992: F, t1484: F, t473: F, t219: F, t4676: F, t571: F, t2021: F, t954: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4855 = 16.0 / 45.0 * t1446 * t1987;
    let t4856 = t1326 * t4637;
    let t4858 = 8.0 / 45.0 * t519 * t4856;
    let t4859 = t1991 * t4615;
    let t4861 = 8.0 / 9.0 * t519 * t4859;
    let t4862 = t4829 * t4633;
    let t4864 = 32.0 / 45.0 * t519 * t4862;
    let t4866 = 8.0 / 27.0 * t1446 * t1992;
    let t4867 = t473 * t1484;
    let t4868 = t4867 * t219;
    let t4869 = t4868 * t4676;
    let t4871 = 16.0 / 27.0 * t571 * t4869;
    let t4872 = t2021 * t954;
    (t4855, t4856, t4858, t4859, t4861, t4862, t4864, t4866, t4867, t4868, t4869, t4871, t4872)
}
