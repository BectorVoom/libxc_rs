//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 777/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk777(t12078: f64, t1397: f64, t12323: f64, t747: f64, t1959: f64, t3730: f64, t3720: f64, t723: f64, t701: f64, t1: f64, t106: f64, t12161: f64, t316: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38770 = t1397 * t12078;
    let t38885 = t12323 * t747;
    let t38892 = t3730 * t1959;
    let t38907 = t3720 * t723;
    let t38912 = t3720 * t701;
    let t38947 = t12161 * t1 * t106 * t316;
    (t38770, t38885, t38892, t38907, t38912, t38947)
}
