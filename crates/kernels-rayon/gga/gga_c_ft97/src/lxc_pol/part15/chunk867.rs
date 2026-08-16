//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 867/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk867(t39673: f64, t1642: f64, t1984: f64, t525: f64, t7954: f64, t378: f64, t7368: f64, t143: f64, t37355: f64, t137: f64, t8906: f64, t135: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39674 = 280.0_f64 / 81.0_f64 * t39673;
    let t39693 = t1642 * t1984;
    let t39725 = t7954 * t525;
    let t39749 = t378 * t7368;
    let t39778 = t143 * t37355;
    let t39801 = 1.0_f64 / t8906 / t137;
    let t39802 = t135 * t39801;
    (t39674, t39693, t39725, t39749, t39778, t39802)
}
