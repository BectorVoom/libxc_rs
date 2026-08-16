//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 461/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk461(t550: f64, t844: f64, t247: f64, t548: f64, t235: f64, t545: f64) -> (f64, f64, f64) {
    let t1404 = t844 * t550;
    let t1405 = t1404 * t247;
    let t1407 = 0.10003937560882938627e-2_f64 * t548 * t1405;
    let t1408 = t545 * t235;
    (t1405, t1407, t1408)
}
