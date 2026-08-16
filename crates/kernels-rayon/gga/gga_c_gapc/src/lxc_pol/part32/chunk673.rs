//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 673/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk673(t442: f64, t5972: f64, t5971: f64, t169: f64, t4605: f64, t5: f64, t521: f64, t1403: f64, t1666: f64, t1388: f64, t515: f64, t1983: f64, t618: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5973 = t5972 * t442;
    let t5974 = t5971 * t5973;
    let t5977 = t169 * t4605;
    let t5979 = t521 * t5;
    let t5983 = t1666 * t1403;
    let t5987 = t1388 * t515;
    let t6055 = t618 * t1983;
    (t5973, t5974, t5977, t5979, t5983, t5987, t6055)
}
