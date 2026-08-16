//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 814/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk814(t2911: f64, t5434: f64, t1013: f64, t4298: f64, t11899: f64, t3020: f64, t5186: f64, t5170: f64, t8688: f64, t2367: f64, t5097: f64, t1220: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15138 = t5434 * t2911;
    let t15142 = t4298 * t1013;
    let t15146 = t11899 * t1013;
    let t15167 = t5186 * t3020;
    let t15174 = t5170 * t8688;
    let t15178 = t2367 * t5097;
    let t15179 = t1220 * t15178;
    (t15138, t15142, t15146, t15167, t15174, t15178, t15179)
}
