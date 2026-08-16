//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 812/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk812(t2398: f64, t2414: f64, t10326: f64, t190: f64, t706: f64, t2258: f64, t750: f64, t157: f64, t36: f64, t10356: f64, t2401: f64, t200: f64, t45: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10432 = 12.0_f64 * t2398 * t2414;
    let t10433 = t190 * t10326;
    let t10435 = 4.0_f64 * t706 * t10433;
    let t10436 = t750 * t2258;
    let t10437 = t706 * t10436;
    let t10438 = 12.0_f64 * t10437;
    let t10439 = t36 * t157;
    let t10440 = t190 * t10356;
    let t10442 = 24.0_f64 * t10439 * t10440;
    let t10443 = t2401 * t750;
    let t10444 = 3.0_f64 * t10443;
    let t10446 = 1.0_f64 / t200 / t45;
    (t10432, t10435, t10438, t10442, t10444, t10446)
}
