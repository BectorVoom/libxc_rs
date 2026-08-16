//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 811/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk811(t11982: f64, t3434: f64, t2210: f64, t160: f64, t7800: f64, t11437: f64, t3439: f64, t1047: f64, t1637: f64, t89: f64, t1053: f64, t2075: f64) -> (f64, f64, f64, f64) {
    let t12742 = t3434 * t11982;
    let t12743 = t2210 * t12742;
    let t12746 = t160 * t7800;
    let t12747 = t12746 * t11437;
    let t12748 = t3439 * t12747;
    let t12752 = t89 * t1637 * t1047;
    let t12754 = t1053 * t2075;
    (t12743, t12748, t12752, t12754)
}
