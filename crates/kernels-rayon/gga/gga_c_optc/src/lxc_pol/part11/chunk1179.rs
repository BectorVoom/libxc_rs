//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1179/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1179(t1582: f64, t1583: f64, t17648: f64, t6: f64, t14863: f64, t4230: f64, t16094: f64, t4536: f64, t15178: f64, t18194: f64, t4215: f64, t15104: f64, t18200: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53399 = t1582 * t1583 * t17648 * t6;
    let t53432 = t4230 * t14863;
    let t53443 = t4536 * t16094;
    let t53445 = t4230 * t15178;
    let t53453 = t4536 * t15178;
    let t53465 = t18194 * t4215;
    let t53470 = t18200 * t15104;
    (t53399, t53432, t53443, t53445, t53453, t53465, t53470)
}
