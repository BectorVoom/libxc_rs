//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 644/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk644(t2251: f64, t2258: f64, t2299: f64, t2306: f64, t633: f64, t637: f64, t77: f64) -> f64 {
    let t2311 = 28.0_f64 / 9.0_f64 * t2299 * t2251 - 4.0_f64 / 3.0_f64 * t633 * t2258 + 28.0_f64 / 9.0_f64 * t2306 * t2251 + 4.0_f64 / 3.0_f64 * t637 * t2258;
    let t2312 = t77 * t2311;
    t2312
}
