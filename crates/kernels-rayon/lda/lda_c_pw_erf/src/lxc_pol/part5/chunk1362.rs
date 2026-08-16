//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1362/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1362(t15311: f64, t15312: f64, t15315: f64, t15316: f64, t19986: f64, t20046: f64, t20056: f64, t20071: f64, t20080: f64, t20088: f64, t20096: f64, t23363: f64, t2991: f64, t3005: f64, t3019: f64, t4305: f64, t5695: f64, t7: f64, t7315: f64, t7378: f64, t8120: f64, t8121: f64, t8122: f64, t8123: f64, t8126: f64, t8130: f64, t8134: f64) -> f64 {
    let t23368 = -t2991 - t8120 + t8121 - 72.0_f64 * t5695 - t15311 - t15312 + t8122 - t8123 + t3005 - t8126 - t15315 - t15316 + t8130 + t4305 - 2.464579730404_f64 * t7315 + t7378 + t3019 + t8134 + t7 * (t19986 + t20046 + t20056 + t20071 + t20080 + t20088 + t20096 + t23363);
    t23368
}
