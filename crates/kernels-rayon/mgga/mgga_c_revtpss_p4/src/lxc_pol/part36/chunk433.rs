//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 433/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk433(t2: f64, t580: f64, t47: f64, t59: f64, t239: f64, t64: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2255 = t2 * t580;
    let t2275 = 1.0_f64 / t47;
    let t2282 = 1.0_f64 / t59;
    let t2289 = t64 * t239;
    let t2290 = 88.0_f64 / 9.0_f64 * t2289;
    let t2297 = t631 * t45;
    let t2299 = 1.0_f64 / t78 / t2297;
    let t2304 = t635 * t57;
    let t2306 = 1.0_f64 / t81 / t2304;
    (t2255, t2275, t2282, t2289, t2290, t2297, t2299, t2304, t2306)
}
