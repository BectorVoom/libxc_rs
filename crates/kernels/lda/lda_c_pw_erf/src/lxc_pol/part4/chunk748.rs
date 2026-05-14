//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 748/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk748<F: Float>(t5175: F, t548: F, t1397: F, t2076: F, t5067: F, t5071: F, t5131: F, t5133: F, t5135: F, t5140: F, t5145: F, t5150: F, t5154: F, t5159: F, t5164: F, t5169: F, t5172: F, t5174: F) -> (F, F, F, F) {
    let t5176 = t548 * t5175;
    let t5177 = 4.0 / 9.0 * t5176;
    let t5179 = 16.0 / 45.0 * t2076 * t1397;
    let t5180 = t5067 + t5071 - t5131 - t5133 + t5135 - t5140 - t5145 + t5150 - t5154 - t5159 - t5164 + t5169 - t5172 + t5174 + t5177 + t5179;
    (t5176, t5177, t5179, t5180)
}
