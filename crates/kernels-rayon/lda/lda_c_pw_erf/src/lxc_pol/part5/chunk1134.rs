//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1134/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1134(t15983: f64, t515: f64, t7661: f64, t16016: f64, t6215: f64, t6875: f64, t2067: f64, t2402: f64, t6611: f64, t835: f64, t16024: f64, t15685: f64, t6230: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20978 = 8.0_f64 / 15.0_f64 * t15983;
    let t20979 = t7661 * t515;
    let t20980 = 4.0_f64 / 45.0_f64 * t20979;
    let t20981 = 4.0_f64 / 15.0_f64 * t16016;
    let t20982 = t6875 * t6215;
    let t20983 = 8.0_f64 / 15.0_f64 * t20982;
    let t20985 = 4.0_f64 / 5.0_f64 * t2402 * t2067;
    let t20987 = 4.0_f64 / 5.0_f64 * t6611 * t835;
    let t20988 = 16.0_f64 / 45.0_f64 * t16024;
    let t20990 = 16.0_f64 / 15.0_f64 * t15685 * t6230;
    (t20978, t20980, t20981, t20983, t20985, t20987, t20988, t20990)
}
