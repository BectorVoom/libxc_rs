//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 254/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk254(t45: f64, t57: f64, t190: f64, t606: f64, t706: f64, t78: f64, t81: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t707 = t190 * t606;
    let t709 = 4.0_f64 * t706 * t707;
    let t712 = piecewise3(t151, 0.0_f64, 4.0_f64 / 3.0_f64 * t78 * t606);
    let t715 = piecewise3(t155, 0.0_f64, -4.0_f64 / 3.0_f64 * t81 * t606);
    let t716 = t712 + t715;
    (t707, t709, t716)
}
