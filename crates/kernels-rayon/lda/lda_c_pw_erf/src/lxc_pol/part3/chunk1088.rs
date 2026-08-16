//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1088/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1088(t12728: f64, t2114: f64, t4039: f64, t9680: f64, t9711: f64, t9714: f64, t9718: f64, t12718: f64, t12719: f64, t12720: f64, t12721: f64, t12722: f64, t12724: f64, t12726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12729 = 4.0_f64 / 15.0_f64 * t12728;
    let t12731 = 4.0_f64 / 5.0_f64 * t2114 * t4039;
    let t12732 = 16.0_f64 / 45.0_f64 * t9680;
    let t12733 = 8.0_f64 / 45.0_f64 * t9711;
    let t12734 = 16.0_f64 / 45.0_f64 * t9714;
    let t12735 = 8.0_f64 / 27.0_f64 * t9718;
    let t12736 = -t12718 + t12719 + t12720 - t12721 - t12722 + t12724 + t12726 - t12729 + t12731 + t12732 - t12733 - t12734 + t12735;
    (t12729, t12731, t12732, t12733, t12734, t12735, t12736)
}
