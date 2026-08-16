//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 767/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk767(t3373: f64, t4092: f64, t4095: f64, t4096: f64, t4099: f64, t4103: f64, t4106: f64, t4113: f64, t5894: f64, t5897: f64, t5898: f64, t5904: f64, t5907: f64, t5911: f64) -> f64 {
    let t7057 = -0.3350512821420176_f64 * t5894 + t5897 + 0.3350512821420176_f64 * t5898 - t3373 + 2.657442045789236_f64 * t5904 - 0.10611888591559791_f64 * t5907 - t5911 - 0.0837628205355044_f64 * t4092 - t4095 - 0.1675256410710088_f64 * t4096 - t4099 + 0.1675256410710088_f64 * t4103 + t4106 + t4113;
    t7057
}
