//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 739/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk739(t1513: f64, t808: f64, t1518: f64, t807: f64, t185: f64, t1931: f64, t230: f64, t610: f64, t838: f64, t2007: f64, t3794: f64, t2119: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4728 = 4.0_f64 / 15.0_f64 * t1513 * t808;
    let t4729 = t1518 * t807;
    let t4730 = t185 * t4729;
    let t4731 = 4.0_f64 / 135.0_f64 * t4730;
    let t4733 = 8.0_f64 / 3.0_f64 * t1931 * t230;
    let t4734 = t838 * t610;
    let t4737 = 16.0_f64 / 45.0_f64 * t3794 * t2007;
    let t4738 = t2119 * t518;
    (t4728, t4729, t4731, t4733, t4734, t4737, t4738)
}
