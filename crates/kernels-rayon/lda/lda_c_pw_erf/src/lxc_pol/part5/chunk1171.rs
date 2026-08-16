//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1171/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1171(t17079: f64, t16602: f64, t2001: f64, t3974: f64, t17102: f64, t17105: f64, t11093: f64, t11097: f64, t11098: f64, t11101: f64, t11104: f64, t19221: f64, t19225: f64, t19228: f64, t19230: f64) -> (f64, f64, f64, f64, f64) {
    let t21378 = 8.0_f64 / 15.0_f64 * t17079;
    let t21381 = 8.0_f64 / 15.0_f64 * t3974 * t16602 * t2001;
    let t21384 = 8.0_f64 / 45.0_f64 * t17102;
    let t21385 = 16.0_f64 / 15.0_f64 * t17105;
    let t21386 = t19221 + 0.18233333333333332_f64 * t19225 + t19228 + 0.36466666666666664_f64 * t19230 - t21378 - t21381 - 8.0_f64 / 405.0_f64 * t11093 + t11097 + 8.0_f64 / 81.0_f64 * t11098 + t11101 - t11104 - t21384 - t21385;
    (t21378, t21381, t21384, t21385, t21386)
}
