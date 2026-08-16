//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 664/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk664(t126: f64, t4864: f64, t102: f64, t457: f64, t1946: f64, t1302: f64, t515: f64, t1709: f64, t442: f64, t1983: f64, t1609: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4865 = t4864 * t126;
    let t4867 = t102 * t457;
    let t4868 = t1946 * t4867;
    let t4882 = t1302 * t515;
    let t4883 = t4882 * t126;
    let t4885 = t1709 * t442;
    let t4893 = t1983 * t442;
    let t4905 = t1609 * t575;
    (t4865, t4867, t4868, t4882, t4883, t4885, t4893, t4905)
}
