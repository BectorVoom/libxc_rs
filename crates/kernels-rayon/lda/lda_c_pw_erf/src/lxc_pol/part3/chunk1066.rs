//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1066/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1066(t12450: f64, t3965: f64, t5141: f64, t12025: f64, t12389: f64, t12476: f64, t348: f64, t12475: f64, t4576: f64, t565: f64, t3384: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12488 = 16.0_f64 / 15.0_f64 * t3965 * t5141 * t12450;
    let t12491 = 16.0_f64 / 3.0_f64 * t3965 * t12025 * t12389;
    let t12492 = t12476 * t348;
    let t12495 = 64.0_f64 / 15.0_f64 * t12475 * t5141 * t12492;
    let t12497 = 8.0_f64 / 15.0_f64 * t565 * t4576;
    let t12498 = t795 * t3384;
    (t12488, t12491, t12492, t12495, t12497, t12498)
}
