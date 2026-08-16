//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 960/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk960(t12531: f64, t527: f64, t1008: f64, t4667: f64, t1106: f64, t1181: f64, t1586: f64, t3391: f64, t3730: f64, t540: f64, t1526: f64, t3573: f64) -> (f64, f64, f64, f64, f64) {
    let t15350 = t12531 * t527;
    let t15362 = t1008 * t4667;
    let t15366 = t3391 * t1181 * t1586 * t1106;
    let t15370 = t3391 * t1181 * t540 * t3730;
    let t15378 = t3573 * t1526;
    (t15350, t15362, t15366, t15370, t15378)
}
