//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1236/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1236(t3736: f64, t7635: f64, t3566: f64, t13036: f64, t13040: f64, t7616: f64, t12854: f64, t29096: f64, t11772: f64, t26865: f64, t3717: f64, t12909: f64, t26866: f64) -> (f64, f64, f64, f64, f64) {
    let t97065 = t7635 * t3736;
    let t97066 = t3566 * t97065;
    let t97133 = t13036 * t7616 * t13040;
    let t97149 = t12854 * t29096;
    let t97173 = t26865 * t11772;
    let t97174 = t3717 * t97173;
    let t97179 = t12909 * t26866;
    (t97066, t97133, t97149, t97174, t97179)
}
