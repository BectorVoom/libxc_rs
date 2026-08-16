//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 889/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk889(t1027: f64, t2735: f64, t2997: f64, t400: f64, t1059: f64, t2993: f64, t2998: f64, t1010: f64, t387: f64, t1039: f64, t1067: f64, t1035: f64, t1070: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8481 = 69.26302359750345_f64 * t400 * t1027 * t2735 * t2997;
    let t8482 = t1059 * t2993;
    let t8486 = t1059 * t2998;
    let t8491 = 4.678578717964164_f64 * t400 * t1010 * t2735 * t387;
    let t8495 = t1067 * t1039;
    let t8497 = t1070 * t1035;
    (t8481, t8482, t8486, t8491, t8495, t8497)
}
