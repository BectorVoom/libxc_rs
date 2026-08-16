//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1188/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1188(t1111: f64, t17352: f64, t9142: f64, t17863: f64, t2586: f64, t1133: f64, t15332: f64, t4363: f64, t1108: f64, t17928: f64, t4386: f64, t9189: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54389 = t1111 * t9142 * t17352;
    let t54391 = t2586 * t17863;
    let t54392 = t1133 * t54391;
    let t54394 = t4363 * t15332;
    let t54408 = t17928 * t1108;
    let t54430 = t4386 * t9189 * t17352;
    (t54389, t54391, t54392, t54394, t54408, t54430)
}
