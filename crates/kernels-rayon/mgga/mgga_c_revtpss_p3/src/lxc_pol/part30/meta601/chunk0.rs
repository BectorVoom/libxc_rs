//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2062/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2062(t11772: f64, t26865: f64, t3717: f64, t13011: f64, t7607: f64, t12909: f64, t26866: f64, t12831: f64, t12917: f64, t26870: f64, t26827: f64, t3678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97173 = t26865 * t11772;
    let t97174 = t3717 * t97173;
    let t97177 = t7607 * t13011;
    let t97179 = t12909 * t26866;
    let t97182 = t12831 * t26866;
    let t97187 = t26870 * t12917;
    let t97191 = t26827 * t3678;
    (t97173, t97174, t97177, t97179, t97182, t97187, t97191)
}
