//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1165/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1165(t43776: f64, t2296: f64, t3241: f64, t11778: f64, t154: f64, t22715: f64, t268: f64, t405: f64, t39267: f64, t404: f64, t410: f64, t407: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43777 = 0.13490888888888888889e1_f64 * t43776;
    let t43791 = 1.0_f64 / t3241 / t2296;
    let t43809 = t154 * t11778;
    let t43819 = t268 * t22715 * t405;
    let t43820 = 280.0_f64 / 81.0_f64 * t43819;
    let t43880 = 1.0_f64 / t410 / t39267 / t404 / 96.0_f64;
    let t43889 = f64::powf(t407, -0.25e1_f64);
    (t43777, t43791, t43809, t43819, t43820, t43880, t43889)
}
