//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 759/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk759(t2269: f64, t2641: f64, t11325: f64, t3916: f64, t322: f64, t8425: f64, t496: f64, t8428: f64, t1587: f64, t429: f64, t7205: f64, t1585: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11518 = t2641 * t2269;
    let t11526 = t3916 * t11325;
    let t11596 = t322 * t8425;
    let t11597 = t496 * t8428;
    let t11603 = t7205 * t429 * t1587;
    let t11604 = t1585 * t11603;
    (t11518, t11526, t11596, t11597, t11603, t11604)
}
