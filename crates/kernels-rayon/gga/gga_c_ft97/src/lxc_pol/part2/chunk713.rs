//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 713/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk713(t2258: f64, t358: f64, t68: f64, t8076: f64, t2993: f64, t7705: f64, t419: f64, t173: f64, t1736: f64, t2984: f64, t11034: f64, t3088: f64) -> (f64, f64, f64, f64, f64) {
    let t11253 = t2258 * t358;
    let t11255 = t68 * t8076 * t11253;
    let t11259 = t7705 * t2993;
    let t11260 = t419 * t11259;
    let t11262 = t173 * t1736;
    let t11263 = t11262 * t2984;
    let t11264 = t419 * t11263;
    let t11265 = 0.56749874115226337448e-2_f64 * t11264;
    let t11266 = t3088 * t11034;
    (t11255, t11260, t11264, t11265, t11266)
}
