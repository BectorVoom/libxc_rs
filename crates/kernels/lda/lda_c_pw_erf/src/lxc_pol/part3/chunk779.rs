//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 779/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk779<F: Float>(t211: F, t5170: F, t1405: F, t822: F, t2071: F, t4567: F, t548: F, t1397: F, t2076: F, t5067: F, t5071: F, t5131: F, t5133: F, t5135: F, t5140: F, t5145: F, t5150: F, t5154: F, t5159: F, t5164: F, t5169: F) -> (F, F, F, F, F, F) {
    let t5172 = F::new(8.0) / F::new(45.0) * t211 * t5170;
    let t5174 = F::new(4.0) / F::new(15.0) * t822 * t1405;
    let t5175 = t4567 * t2071;
    let t5176 = t548 * t5175;
    let t5177 = F::new(4.0) / F::new(9.0) * t5176;
    let t5179 = F::new(16.0) / F::new(45.0) * t2076 * t1397;
    let t5180 = t5067 + t5071 - t5131 - t5133 + t5135 - t5140 - t5145 + t5150 - t5154 - t5159 - t5164 + t5169 - t5172 + t5174 + t5177 + t5179;
    (t5172, t5174, t5175, t5177, t5179, t5180)
}
