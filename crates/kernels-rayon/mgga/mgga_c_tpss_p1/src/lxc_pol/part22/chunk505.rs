//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 505/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk505(t2025: f64, t38: f64, t45: f64, t606: f64, t78: f64, t57: f64, t610: f64, t81: f64, t1985: f64, t1992: f64, t608: f64, t612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2026 = t38 * t2025;
    let t2031 = t606 * t45;
    let t2033 = 1.0_f64 / t78 / t2031;
    let t2038 = t610 * t57;
    let t2040 = 1.0_f64 / t81 / t2038;
    let t2045 = 28.0_f64 / 9.0_f64 * t2033 * t1985 - 4.0_f64 / 3.0_f64 * t608 * t1992 + 28.0_f64 / 9.0_f64 * t2040 * t1985 + 4.0_f64 / 3.0_f64 * t612 * t1992;
    (t2026, t2031, t2033, t2038, t2040, t2045)
}
