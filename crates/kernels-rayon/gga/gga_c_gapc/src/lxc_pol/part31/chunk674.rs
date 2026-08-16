//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 674/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk674(t1698: f64, t442: f64, t619: f64, t457: f64, t681: f64, t1903: f64, t1908: f64, t198: f64, t137: f64, t567: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t5189 = t1698 * t442;
    let t5190 = t619 * t5189;
    let t5199 = t681 * t457;
    let t5211 = t1903 * pi;
    let t5214 = t198 * t1908;
    let t5215 = t5214 * t681;
    let t5216 = t567 * t137;
    (t5190, t5199, t5211, t5214, t5215, t5216)
}
