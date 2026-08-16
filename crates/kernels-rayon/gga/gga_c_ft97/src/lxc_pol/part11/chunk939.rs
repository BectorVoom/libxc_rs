//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 939/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk939(t38111: f64, t528: f64, t118: f64, t37993: f64, t37640: f64, t38062: f64, t38046: f64, t38050: f64, t38069: f64, t38077: f64, t38081: f64, t38084: f64, t38088: f64, t38090: f64, t38101: f64, t38108: f64, t39533: f64) -> (f64, f64, f64) {
    let t39535 = t528 * t38111;
    let t39538 = 1.0_f64 / t118 / t37993;
    let t39539 = t39538 * t37640;
    let t39546 = 0.14978012345679012345e1_f64 * t38062;
    let t39550 = -0.51995099999999999998e1_f64 * t38101 + 0.11554466666666666666e1_f64 * t38108 - 0.352131e0_f64 * t39533 + 0.234754e0_f64 * t39535 - 0.44016375e0_f64 * t39539 + 0.38514888888888888888e0_f64 * t38046 - 0.11554466666666666666e1_f64 * t38050 - 0.9628722222222222222e0_f64 * t38069 + 0.34663399999999999999e1_f64 * t38077 - 0.38514888888888888888e0_f64 * t38084 + t39546 - 0.28886166666666666666e0_f64 * t38081 + 0.34663399999999999999e1_f64 * t38088 + 0.59912049382716049381e0_f64 * t38090;
    (t39535, t39539, t39550)
}
