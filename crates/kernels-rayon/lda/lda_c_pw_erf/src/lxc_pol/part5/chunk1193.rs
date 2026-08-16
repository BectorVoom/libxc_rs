//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1193/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1193(t668: f64, t8025: f64, t17458: f64, t17461: f64, t571: f64, t7723: f64, t9278: f64, t108: f64, t15060: f64, t19249: f64, t21605: f64, t21608: f64, t21611: f64, t21614: f64, t21617: f64, t21622: f64, t21624: f64, t21657: f64, t267: f64) -> (f64, f64, f64, f64) {
    let t21661 = t8025 * t668;
    let t21664 = 16.0_f64 / 15.0_f64 * t17458;
    let t21665 = 8.0_f64 / 45.0_f64 * t17461;
    let t21667 = t571 * t9278 * t7723;
    let t21668 = 8.0_f64 / 27.0_f64 * t21667;
    let t21669 = t21605 + t21608 - t21611 - t21614 + t21617 - t21622 + 0.09973633333333333_f64 * t19249 + t21624 - t21657 * t108 * t267 / 15.0_f64 - 2.0_f64 / 45.0_f64 * t21661 - 0.040518518518518516_f64 * t15060 + t21664 - t21665 - t21668;
    (t21664, t21665, t21668, t21669)
}
