//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 884/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk884(t45: f64, t57: f64, t10468: f64, t190: f64, t606: f64, t80: f64, t10326: f64, t10356: f64, t2258: f64, t633: f64, t766: f64, t83: f64, t637: f64, t770: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10469 = t10468 * t190;
    let t10472 = t80 * t606;
    let t10478 = piecewise3(t151, 0.0_f64, 8.0_f64 / 27.0_f64 * t633 * t10356 - 2.0_f64 / 3.0_f64 * t10472 * t2258 + 2.0_f64 / 3.0_f64 * t766 * t10326);
    let t10481 = t83 * t606;
    let t10487 = piecewise3(t155, 0.0_f64, -8.0_f64 / 27.0_f64 * t637 * t10356 - 2.0_f64 / 3.0_f64 * t10481 * t2258 - 2.0_f64 / 3.0_f64 * t770 * t10326);
    (t10469, t10478, t10487)
}
