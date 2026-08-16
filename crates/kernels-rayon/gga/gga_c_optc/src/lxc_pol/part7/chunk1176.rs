//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1176/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1176(t2364: f64, t7239: f64, t7294: f64, t212: f64, t2263: f64, t362: f64, t508: f64, t896: f64, t769: f64, t2640: f64, t7470: f64, t2643: f64, t7266: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24386 = t2364 * t7239;
    let t24388 = t2364 * t7294;
    let t24391 = 1.0_f64 / t212 / t2263;
    let t24392 = t24391 * t362;
    let t24407 = t508 * t896;
    let t24408 = t24407 * t769;
    let t24410 = t2640 * t24408 * t7470;
    let t24412 = t2643 * t7266;
    (t24386, t24388, t24391, t24392, t24410, t24412)
}
