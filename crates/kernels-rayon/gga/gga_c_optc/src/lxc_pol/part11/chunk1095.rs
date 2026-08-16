//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1095/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1095(t2640: f64, t31579: f64, t4947: f64, t2678: f64, t2679: f64, t40326: f64, t4975: f64, t7878: f64, t893: f64, t4961: f64, t896: f64, t4929: f64, t530: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41526 = t2640 * t31579 * t4947;
    let t41585 = t2678 * t40326 * t2679;
    let t41756 = t7878 * t4975;
    let t41757 = t893 * t41756;
    let t41818 = t896 * t4961;
    let t41832 = t862 * t530 * t4929;
    (t41526, t41585, t41756, t41757, t41818, t41832)
}
