//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 592/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk592(t2529: f64, t4854: f64, t837: f64, t845: f64, t2480: f64, t3640: f64, t4770: f64, t4774: f64, t4778: f64, t1354: f64) -> (f64, f64, f64, f64) {
    let t4856 = t2529 * t4854 * t837;
    let t4858 = 0.11696446794910408142e1_f64 * t845 * t4856;
    let t4863 = t2480 + 0.11415555555555555555e-1_f64 * t3640 - 0.11415555555555555555e-1_f64 * t4770 + 0.34246666666666666666e-1_f64 * t4774 - 0.17123333333333333333e-1_f64 * t4778;
    let t4868 = t1354 * t1354;
    (t4856, t4858, t4863, t4868)
}
