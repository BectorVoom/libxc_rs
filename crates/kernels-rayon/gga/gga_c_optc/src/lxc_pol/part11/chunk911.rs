//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 911/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk911(t3927: f64, t4768: f64, t3608: f64, t17118: f64, t8216: f64, t4961: f64, t8201: f64, t3885: f64, t3623: f64, t4963: f64, t16988: f64, t2669: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17134 = t3927 * t4768;
    let t17135 = t3608 * t17134;
    let t17138 = t17118 * t8216;
    let t17141 = t8201 * t4961;
    let t17142 = t3885 * t17141;
    let t17145 = t3623 * t4963;
    let t17148 = t2669 * t16988;
    (t17134, t17135, t17138, t17141, t17142, t17145, t17148)
}
