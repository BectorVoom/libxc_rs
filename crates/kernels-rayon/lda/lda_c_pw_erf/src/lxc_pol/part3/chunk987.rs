//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 987/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk987(t1549: f64, t5495: f64, t159: f64, t285: f64, t462: f64, t4713: f64, t4422: f64, t477: f64, t1128: f64, t1896: f64, t405: f64, t5669: f64) -> (f64, f64, f64, f64, f64) {
    let t11486 = t1549 * t5495;
    let t11495 = t462 * t4713 * t159 * t285;
    let t11498 = t4422 * t477 * t285;
    let t11499 = 0.0017434044910732151_f64 * t11498;
    let t11501 = t1896 * t1128 * t285;
    let t11507 = t405 * t5669;
    (t11486, t11495, t11499, t11501, t11507)
}
