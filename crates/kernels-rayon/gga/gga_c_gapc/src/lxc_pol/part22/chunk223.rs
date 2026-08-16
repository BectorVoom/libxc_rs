//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 223/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk223(t829: f64, t831: f64, t766: f64, t798: f64, t297: f64, t332: f64, t268: f64, t9: f64, t22: f64, t760: f64, t768: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t832 = t829 * t831;
    let t835 = t766 * t798;
    let t836 = t297 * t332;
    let t837 = t9 * t268;
    let t841 = t22 * t760;
    let t845 = t768 * t786;
    (t832, t835, t836, t837, t841, t845)
}
