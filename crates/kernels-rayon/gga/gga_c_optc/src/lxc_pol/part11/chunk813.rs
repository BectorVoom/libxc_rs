//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 813/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk813(t4215: f64, t5250: f64, t5257: f64, t1107: f64, t190: f64, t5245: f64, t5243: f64, t11782: f64, t5228: f64, t4297: f64, t5087: f64, t9254: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15099 = t5250 * t4215;
    let t15101 = t5257 * t4215;
    let t15104 = t1107 * t190 * t5245;
    let t15105 = t5243 * t15104;
    let t15107 = t11782 * t5228;
    let t15108 = t4297 * t15107;
    let t15122 = t5087 * t9254;
    (t15099, t15101, t15104, t15105, t15107, t15108, t15122)
}
