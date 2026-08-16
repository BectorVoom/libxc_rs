//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1353/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1353(t11262: f64, t1251: f64, t1247: f64, t1284: f64, t3566: f64, t3624: f64, t126: f64, t482: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12904 = t11262 * t1251;
    let t12905 = t1247 * t12904;
    let t12909 = t3566 * t1284;
    let t12910 = t12909 * t3624;
    let t12915 = t126 * t482;
    let t12916 = t828 * t12915;
    (t12904, t12905, t12909, t12910, t12915, t12916)
}
