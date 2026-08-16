//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2172/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2172(t57: f64, t22671: f64, t22688: f64, t4335: f64, t5825: f64, t637: f64, t770: f64, t23138: f64, zeta_threshold: f64) -> f64 {
    let t155 = t57 <= zeta_threshold;
    let t23146 = piecewise3(t155, 0.0_f64, -8.0_f64 / 27.0_f64 * t637 * t22688 - 2.0_f64 / 3.0_f64 * t4335 * t5825 - 2.0_f64 / 3.0_f64 * t770 * t22671);
    let t23148 = t23138 / 2.0_f64 + t23146 / 2.0_f64;
    t23148
}
