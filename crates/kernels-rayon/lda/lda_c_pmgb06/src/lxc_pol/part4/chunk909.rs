//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 909/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk909(t2653: f64, t489: f64, t161: f64, t2630: f64, t435: f64, t132: f64, t2624: f64, t2018: f64, t831: f64, t2649: f64, t2015: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6610 = t489 * t2653;
    let t6611 = t161 * t6610;
    let t6612 = 2.0_f64 / 45.0_f64 * t6611;
    let t6613 = t435 * t2630;
    let t6614 = t132 * t6613;
    let t6615 = 2.0_f64 / 45.0_f64 * t6614;
    let t6616 = t489 * t2624;
    let t6617 = t161 * t6616;
    let t6618 = t6617 / 45.0_f64;
    let t6619 = t831 * t2018;
    let t6620 = 2.0_f64 / 45.0_f64 * t6619;
    let t6621 = t435 * t2649;
    let t6622 = t132 * t6621;
    let t6623 = t6622 / 45.0_f64;
    let t6624 = t802 * t2015;
    (t6610, t6612, t6613, t6615, t6616, t6618, t6620, t6621, t6623, t6624)
}
