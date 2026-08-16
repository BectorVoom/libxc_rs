//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 571/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk571(t1053: f64, t1659: f64, t225: f64, t4743: f64, t366: f64, t1065: f64, t2857: f64, t4181: f64, t1042: f64, t2852: f64, t3181: f64, t1592: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4794 = t1659 * t1053;
    let t4797 = t4743 * t225;
    let t4798 = t4797 * t366;
    let t4801 = t1065 * t2857;
    let t4802 = t4801 * t4181;
    let t4803 = t1042 * t4802;
    let t4806 = t3181 * t2852;
    let t4807 = t4806 * t4181;
    let t4808 = t1042 * t4807;
    let t4816 = t3109 * t1592;
    (t4794, t4797, t4798, t4803, t4808, t4816)
}
