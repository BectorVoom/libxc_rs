//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1328/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1328(t19: f64, t5944: f64, t729: f64, t734: f64, t11468: f64, t11470: f64, t11472: f64, t11475: f64, t11476: f64, t11516: f64, t11530: f64, t11556: f64, t11599: f64, t14423: f64, t14425: f64, t14426: f64, t14475: f64, t14517: f64, t14908: f64, t15281: f64, t312: f64, t8414: f64, t8417: f64, t8423: f64, t8427: f64, t8432: f64, t8437: f64, t8445: f64, t8449: f64, t8469: f64) -> f64 {
    let t15288 = t5944 * t729 * t19 * t734;
    let t15290 = t8414 + t8417 + t11468 - t11470 + t11472 + t11475 + t8423 - t8427 + t8432 + t8437 - t11476 + t8445 - t8449 - (t11516 + t11530 + t11556 + t11599 + t14475 + t14517 + t14908 + t15281) * t312 + t14423 - 1.232289865202_f64 * t15288 + t14425 - t14426 + t8469;
    t15290
}
