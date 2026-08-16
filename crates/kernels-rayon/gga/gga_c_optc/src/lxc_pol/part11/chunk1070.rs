//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1070/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1070(t23: f64, t3086: f64, t191: f64, t1574: f64, t490: f64, t9226: f64, t1220: f64, t1578: f64, t3902: f64, t1570: f64, t9227: f64, t1523: f64, t8996: f64) -> (f64, f64, f64, f64, f64) {
    let t34028 = t23 * t3086;
    let t34029 = t34028 * t191;
    let t34107 = t490 * t1574 * t9226;
    let t34301 = t1220 * t3902 * t1578;
    let t34309 = t1570 * t9227;
    let t34319 = t1523 * t8996;
    (t34029, t34107, t34301, t34309, t34319)
}
