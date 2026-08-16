//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1042/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1042(t25423: f64, t8126: f64, t19: f64, t769: f64, t3906: f64, t2662: f64, t322: f64, t8192: f64) -> (f64, f64, f64, f64, f64) {
    let t25424 = t8126 * t25423;
    let t25427 = t19 * t769;
    let t25453 = t3906 * t25423;
    let t25458 = t2662 * t25423;
    let t25560 = t8192 * t322;
    (t25424, t25427, t25453, t25458, t25560)
}
