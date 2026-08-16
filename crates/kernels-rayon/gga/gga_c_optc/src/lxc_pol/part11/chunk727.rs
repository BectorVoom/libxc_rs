//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 727/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk727(t376: f64, t383: f64, t3145: f64, t56: f64, t136: f64, t3086: f64, t209: f64, t371: f64, t681: f64) -> (f64, f64, f64, f64) {
    let t8617 = 1.0_f64 / t376 / t383 / 4.0_f64;
    let t8620 = t56 * t3145;
    let t8634 = t136 * t3086;
    let t8639 = t209 * t681 * t371;
    (t8617, t8620, t8634, t8639)
}
