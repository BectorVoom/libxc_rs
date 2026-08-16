//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 665/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk665(t457: f64, t662: f64, t3141: f64, t481: f64, t505: f64, t1906: f64, t674: f64, t682: f64, t1927: f64, t583: f64, t623: f64, t1393: f64, t515: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5726 = t662 * t457;
    let t5727 = t3141 * t5726;
    let t5730 = t481 * t505;
    let t5741 = t1906 * t674;
    let t5742 = t682 * t457;
    let t5743 = t5741 * t5742;
    let t5799 = t1927 * t583;
    let t5803 = t1927 * t623;
    let t5856 = t1393 * t515;
    (t5727, t5730, t5741, t5743, t5799, t5803, t5856)
}
