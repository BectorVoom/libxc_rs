//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 746/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk746(t71660: f64, t70071: f64, t70078: f64, t70082: f64, t14494: f64, t874: f64, t14563: f64, t2160: f64, t638: f64, t14559: f64, t70188: f64, t70237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71661 = 0.34200192530023447503e-6_f64 * t71660;
    let t71670 = 0.66671395154821946452e-1_f64 * t70071;
    let t71671 = 0.39032073591371545778e-3_f64 * t70078;
    let t71672 = 0.30487649791575028312e-3_f64 * t70082;
    let t71704 = t874 * t14494;
    let t71717 = t638 * t2160 * t14563;
    let t71720 = t638 * t2160 * t14559;
    let t71727 = 0.46328831667894726564e-5_f64 * t70188;
    let t71744 = 0.60975299583150056624e-3_f64 * t70237;
    (t71661, t71670, t71671, t71672, t71704, t71717, t71720, t71727, t71744)
}
