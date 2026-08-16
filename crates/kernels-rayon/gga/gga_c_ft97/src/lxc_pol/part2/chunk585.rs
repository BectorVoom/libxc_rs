//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 585/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk585(t2: f64, t2486: f64, t3691: f64, t2493: f64, t3695: f64, t737: f64, t3700: f64, t18: f64, t738: f64, t1152: f64, t458: f64, t3713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3910 = t2486 * t2;
    let t3911 = t3910 * t3691;
    let t3914 = t2493 * t3695;
    let t3917 = t737 * t2;
    let t3918 = t3917 * t3700;
    let t3921 = t738 * t18;
    let t3922 = t737 * t3921;
    let t3925 = t458 * t1152;
    let t3927 = t2493 * t3713;
    (t3910, t3911, t3914, t3917, t3918, t3921, t3922, t3925, t3927)
}
