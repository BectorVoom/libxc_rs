//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 762/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk762(t1561: f64, t2885: f64, t1574: f64, t2838: f64, t490: f64, t1113: f64, t23: f64, t191: f64, t24: f64, t3086: f64, t496: f64, t8414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11700 = t1561 * t2885;
    let t11760 = t1574 * t2838;
    let t11761 = t490 * t11760;
    let t11781 = t23 * t1113;
    let t11782 = t11781 * t191;
    let t11885 = t24 * t3086;
    let t11894 = t496 * t8414;
    (t11700, t11760, t11761, t11781, t11782, t11885, t11894)
}
