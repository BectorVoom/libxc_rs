//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 645/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk645(t2268: f64, t8675: f64, t2253: f64, t2273: f64, t2281: f64, t71: f64, t118: f64, t7911: f64, t7944: f64, t2296: f64, t3626: f64, t70: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8676 = t8675 * t2268;
    let t8678 = t2253 * t2273;
    let t8680 = t71 * t2281;
    let t8690 = 1.0_f64 / t118 / t7911;
    let t8698 = 0.44934037037037037036e0_f64 * t7944;
    let t8714 = t2253 * t2296;
    let t8715 = t3626 * t70;
    (t8676, t8678, t8680, t8690, t8698, t8714, t8715)
}
