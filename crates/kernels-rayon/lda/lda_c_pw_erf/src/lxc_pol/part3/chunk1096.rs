//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1096/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1096(t12814: f64, t3863: f64, t5306: f64, t571: f64, t3859: f64, t4628: f64, t519: f64, t11777: f64, t1326: f64, t12788: f64, t12793: f64, t12796: f64, t12799: f64, t12801: f64, t12803: f64, t12807: f64, t12810: f64, t12812: f64) -> (f64, f64, f64, f64, f64) {
    let t12815 = 8.0_f64 / 135.0_f64 * t12814;
    let t12817 = t571 * t3863 * t5306;
    let t12818 = 16.0_f64 / 45.0_f64 * t12817;
    let t12820 = t519 * t3859 * t4628;
    let t12821 = 16.0_f64 / 15.0_f64 * t12820;
    let t12824 = 32.0_f64 / 15.0_f64 * t519 * t1326 * t11777;
    let t12825 = t12788 + t12793 + t12796 + t12799 - t12801 + t12803 - t12807 - t12810 - t12812 + t12815 + t12818 + t12821 - t12824;
    (t12815, t12818, t12821, t12824, t12825)
}
