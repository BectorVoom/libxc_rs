//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 940/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk940(t14239: f64, t5741: f64, t6844: f64, t72: f64, t686: f64, t4101: f64, t6874: f64, t545: f64, t6888: f64, t869: f64, t689: f64, t22005: f64, t4003: f64, t5744: f64) -> (f64, f64, f64, f64, f64) {
    let t22329 = t14239 * t5741;
    let t22331 = t6844 * t72;
    let t22332 = t22331 * t686;
    let t22333 = t4101 * t22332;
    let t22335 = t6874 * t72;
    let t22336 = t22335 * t686;
    let t22337 = t4101 * t22336;
    let t22351 = t545 * t6888;
    let t22352 = t869 * t22351;
    let t22353 = t689 * t22352;
    let t22361 = t5744 * t22005 * t4003;
    (t22329, t22333, t22337, t22353, t22361)
}
