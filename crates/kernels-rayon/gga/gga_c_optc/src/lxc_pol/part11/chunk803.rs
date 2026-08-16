//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 803/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk803(t1036: f64, t5165: f64, t2367: f64, t5232: f64, t1220: f64, t5202: f64, t8749: f64, t5148: f64, t531: f64) -> (f64, f64, f64, f64, f64) {
    let t14852 = t5165 * t1036;
    let t14863 = t2367 * t5232;
    let t14864 = t1220 * t14863;
    let t14871 = t8749 * t5202;
    let t14881 = t531 * t5148;
    (t14852, t14863, t14864, t14871, t14881)
}
