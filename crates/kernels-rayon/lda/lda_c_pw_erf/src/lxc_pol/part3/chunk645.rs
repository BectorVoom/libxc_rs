//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 645/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk645(t1308: f64, t3828: f64, t571: f64, t1485: f64, t581: f64, t1352: f64, t593: f64, t1356: f64, t549: f64, t1319: f64, t1318: f64, t3619: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3829 = t1308 * t3828;
    let t3831 = 4.0_f64 / 15.0_f64 * t571 * t3829;
    let t3832 = t1485 * t581;
    let t3833 = t1352 * t593;
    let t3834 = t3832 * t3833;
    let t3836 = 4.0_f64 / 9.0_f64 * t571 * t3834;
    let t3837 = t1356 * t549;
    let t3838 = t1319 * t3837;
    let t3840 = 16.0_f64 / 15.0_f64 * t1318 * t3838;
    let t3841 = t1319 * t3619;
    (t3829, t3831, t3832, t3833, t3834, t3836, t3837, t3838, t3840, t3841)
}
