//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1164/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1164<F: Float>(t2146: F, t5375: F, t17093: F, t17095: F, t17100: F, t17103: F, t17104: F, t17106: F, t17108: F, t17110: F, t17113: F, t17115: F, t17118: F, t17121: F, t17124: F, t17128: F, t17130: F, t17132: F, t225: F, t231: F) -> (F, F) {
    let t17134 = 8.0 / 15.0 * t2146 * t5375;
    let t17135 = t17093 + 4.0 / 3.0 * t17095 * t225 * t231 - t17100 - t17103 - t17104 + t17106 + t17108 + t17110 + t17113 + t17115 + t17118 + t17121 - t17124 - t17128 - t17130 - t17132 + t17134;
    (t17134, t17135)
}
