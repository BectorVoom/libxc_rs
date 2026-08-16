//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 702/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk702(t195: f64, t287: f64, t362: f64, t357: f64, t355: f64, t2471: f64, t261: f64) -> (f64, f64, f64, f64) {
    let t7328 = t195 * t287;
    let t7329 = t7328 * t362;
    let t7330 = t357 * t7329;
    let t7332 = 5.0_f64 / 27.0_f64 * t355 * t7330;
    let t7341 = 1.0_f64 / t2471 / t261;
    (t7328, t7330, t7332, t7341)
}
