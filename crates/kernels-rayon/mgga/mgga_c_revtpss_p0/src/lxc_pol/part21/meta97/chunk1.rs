//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 662/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk662(t45: f64, t57: f64, t190: f64, t2258: f64, t706: f64, t2251: f64, t766: f64, t80: f64, t770: f64, t83: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2414 = t190 * t2258;
    let t2416 = 4.0_f64 * t706 * t2414;
    let t2422 = piecewise3(t151, 0.0_f64, -2.0_f64 / 9.0_f64 * t80 * t2251 + 2.0_f64 / 3.0_f64 * t766 * t2258);
    let t2428 = piecewise3(t155, 0.0_f64, -2.0_f64 / 9.0_f64 * t83 * t2251 - 2.0_f64 / 3.0_f64 * t770 * t2258);
    let t2430 = t2422 / 2.0_f64 + t2428 / 2.0_f64;
    (t2414, t2416, t2430)
}
