//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1180/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1180(t1220: f64, t17442: f64, t2367: f64, t1217: f64, t1218: f64, t17926: f64, t18178: f64, t2911: f64, t1199: f64, t17574: f64, t1213: f64, t18204: f64, t490: f64) -> (f64, f64, f64, f64, f64) {
    let t53494 = t1220 * t2367 * t17442;
    let t53498 = t1217 * t1218 * t17926;
    let t53510 = t18178 * t2911;
    let t53612 = t17574 * t1199;
    let t53769 = t490 * t18204 * t1213;
    (t53494, t53498, t53510, t53612, t53769)
}
