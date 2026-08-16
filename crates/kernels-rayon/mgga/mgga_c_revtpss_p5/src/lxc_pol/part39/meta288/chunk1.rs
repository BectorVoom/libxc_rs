//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1036/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1036(t2389: f64, t705: f64, t2258: f64, t750: f64, t706: f64, t157: f64, t36: f64, t2401: f64, t200: f64, t45: f64, t202: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10428 = t705 * t2389;
    let t10436 = t750 * t2258;
    let t10437 = t706 * t10436;
    let t10439 = t36 * t157;
    let t10443 = t2401 * t750;
    let t10446 = 1.0_f64 / t200 / t45;
    let t10457 = 1.0_f64 / t202 / t57;
    (t10428, t10437, t10439, t10443, t10446, t10457)
}
