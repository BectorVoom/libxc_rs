//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 579/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk579(t3826: f64, t661: f64, t1140: f64, t1882: f64, t1131: f64, t713: f64, t2574: f64, t265: f64, t766: f64, t729: f64, t762: f64, t1091: f64, t724: f64, t773: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3827 = t661 * t3826;
    let t3835 = t1882 * t1140;
    let t3837 = t1131 * t713;
    let t3839 = t2574 * t265 * t3837;
    let t3842 = t1131 * t766;
    let t3844 = t729 * t762 * t3842;
    let t3848 = t724 * t773 * t1091;
    (t3827, t3835, t3837, t3839, t3842, t3844, t3848)
}
