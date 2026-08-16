//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 965/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk965(t11730: f64, t2578: f64, t3768: f64, t761: f64, t334: f64, t11533: f64, t277: f64, t3781: f64, t3757: f64, t920: f64, t129: f64, t7073: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11731 = t2578 * t11730;
    let t11733 = t761 * t3768;
    let t11734 = t11733 * t334;
    let t11736 = t277 * t11533;
    let t11737 = t11736 * t3781;
    let t11739 = t3757 * t920;
    let t11741 = t7073 * t129;
    (t11731, t11733, t11734, t11736, t11737, t11739, t11741)
}
