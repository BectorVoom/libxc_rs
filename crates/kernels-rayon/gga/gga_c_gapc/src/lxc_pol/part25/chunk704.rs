//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 704/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk704(t332: f64, t918: f64, t2776: f64, t442: f64, t2642: f64, t959: f64, t2206: f64, t871: f64) -> (f64, f64, f64, f64) {
    let t7418 = t918 * t332;
    let t7419 = t2776 * t442;
    let t7420 = t7418 * t7419;
    let t7442 = t2642 * t959 * t332;
    let t7451 = t871 * t2206;
    (t7418, t7420, t7442, t7451)
}
