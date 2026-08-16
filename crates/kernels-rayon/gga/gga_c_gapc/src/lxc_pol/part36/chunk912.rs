//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 912/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk912(t1125: f64, t2822: f64, t3649: f64, t423: f64, t1459: f64, t3652: f64, t1423: f64, t1464: f64, t3651: f64, t632: f64, t996: f64, t3634: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11155 = t1125 * t2822;
    let t11181 = t3649 * t423;
    let t11182 = t11181 * t1459;
    let t11183 = t11182 * t3652;
    let t11185 = t1423 * t1464;
    let t11186 = t3651 * t11185;
    let t11188 = t996 * t632;
    let t11189 = t3634 * t458;
    (t11155, t11181, t11182, t11183, t11185, t11186, t11188, t11189)
}
