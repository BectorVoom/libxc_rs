//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 482/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk482(t45: f64, t57: f64, t2371: f64, t508: f64, t200: f64, t2251: f64, t2258: f64, t78: f64, t202: f64, t81: f64, t162: f64, t187: f64, t205: f64, t262: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2372 = t508 * t2371;
    let t2375 = 1.0_f64 / t200;
    let t2381 = piecewise3(t151, 0.0_f64, 4.0_f64 / 9.0_f64 * t2375 * t2251 + 4.0_f64 / 3.0_f64 * t78 * t2258);
    let t2382 = 1.0_f64 / t202;
    let t2388 = piecewise3(t155, 0.0_f64, 4.0_f64 / 9.0_f64 * t2382 * t2251 - 4.0_f64 / 3.0_f64 * t81 * t2258);
    let t2389 = t2381 + t2388;
    let t2390 = t2389 * t162;
    let t2392 = 0.19751673498613801407e-1_f64 * t2390 * t187;
    let t2393 = t205 * t262;
    (t2372, t2375, t2382, t2389, t2390, t2392, t2393)
}
