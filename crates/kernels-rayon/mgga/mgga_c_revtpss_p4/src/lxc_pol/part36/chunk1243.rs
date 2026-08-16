//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1243/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1243(t25946: f64, t97916: f64, t136: f64, t2457: f64, t7929: f64, t25944: f64, t2470: f64, t27887: f64, t7284: f64, t1955: f64, t27836: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97917 = t97916 * t25946;
    let t97922 = t7929 * t136 * t2457;
    let t97923 = t25944 * t97922;
    let t97925 = t27887 * t2470;
    let t97926 = t7284 * t97925;
    let t97933 = t1955 * t27836 * t4075;
    (t97917, t97922, t97923, t97925, t97926, t97933)
}
