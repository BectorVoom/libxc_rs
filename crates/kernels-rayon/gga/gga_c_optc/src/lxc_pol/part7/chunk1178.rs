//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1178/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1178(t549: f64, t935: f64, t24432: f64, t7492: f64, t297: f64, t770: f64, t22: f64, t7856: f64, t7256: f64, t19: f64, t2595: f64, t2670: f64) -> (f64, f64, f64, f64, f64) {
    let t24433 = t549 * t935;
    let t24434 = t24432 * t24433;
    let t24438 = t7492 * t24433;
    let t24442 = t935 * t297;
    let t24443 = t24442 * t770;
    let t24447 = t22 * t7856;
    let t24448 = t24447 * t7256;
    let t24458 = t2595 * t2670 * t19;
    (t24434, t24438, t24443, t24448, t24458)
}
