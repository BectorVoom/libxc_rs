//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 829/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk829(t2621: f64, t2624: f64, t2628: f64, t2641: f64, t2826: f64, t2654: f64, t2657: f64, t2839: f64, t2668: f64, t2694: f64, t123: f64, t721: f64, t776: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11514 = 0.86748650402413918736e-1_f64 * t2621;
    let t11515 = 0.1301229756036208781e0_f64 * t2624;
    let t11516 = 0.14035736694323150897e2_f64 * t2628;
    let t11520 = 48.0_f64 * t2641;
    let t11522 = 4.0_f64 * t2826;
    let t11525 = 0.1929837539843104208e3_f64 * t2654;
    let t11526 = 24.0_f64 * t2657;
    let t11529 = 0.4155806185363551302e3_f64 * t2839;
    let t11534 = 24.0_f64 * t2668;
    let t11536 = 4.0_f64 * t2694;
    let t11545 = 0.22911460125803964958e1_f64 * t721 * t123 * t776 * t780;
    (t11514, t11515, t11516, t11520, t11522, t11525, t11526, t11529, t11534, t11536, t11545)
}
