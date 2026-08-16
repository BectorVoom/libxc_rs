//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 309/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk309(t1405: f64, t548: f64, t235: f64, t545: f64, t239: f64, t820: f64, t530: f64, t549: f64) -> (f64, f64, f64, f64) {
    let t1407 = 0.10003937560882938627e-2_f64 * t548 * t1405;
    let t1408 = t545 * t235;
    let t1410 = t820 * t1408 * t239;
    let t1412 = 1.0_f64 / t549 / t530;
    (t1407, t1408, t1410, t1412)
}
