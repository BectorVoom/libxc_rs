//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1087/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1087(t1513: f64, t2134: f64, t9627: f64, t9629: f64, t9645: f64, t9647: f64, t2127: f64, t5069: f64, t2131: f64, t211: f64, t5030: f64, t514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12717 = t1513 * t2134;
    let t12718 = 8.0_f64 / 15.0_f64 * t12717;
    let t12719 = 8.0_f64 / 15.0_f64 * t9627;
    let t12720 = 8.0_f64 / 15.0_f64 * t9629;
    let t12721 = 16.0_f64 / 135.0_f64 * t9645;
    let t12722 = 16.0_f64 / 15.0_f64 * t9647;
    let t12723 = t5069 * t2127;
    let t12724 = 16.0_f64 / 15.0_f64 * t12723;
    let t12726 = 8.0_f64 / 5.0_f64 * t5069 * t2131;
    let t12728 = t211 * t514 * t5030;
    (t12718, t12719, t12720, t12721, t12722, t12724, t12726, t12728)
}
