//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 886/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk886<F: Float>(t1064: F, t2344: F, t1067: F, t344: F, t6069: F, t479: F, t7032: F, t145: F, t6039: F, t164: F, t7045: F, t2660: F, t610: F, t10605: F, t2543: F, t571: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15455 = t1064 * t2344;
    let t15457 = t1067 * t2344;
    let t15461 = t344 * t6069;
    let t15481 = t7032 * t479;
    let t15483 = t145 * t6039;
    let t15484 = t15483 * t164;
    let t15486 = t7045 * t479;
    let t15501 = t2660 * t610;
    let t15521 = t571 * t10605 * t2543;
    (t15455, t15457, t15461, t15481, t15483, t15484, t15486, t15501, t15521)
}
