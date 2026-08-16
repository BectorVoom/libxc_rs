//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 815/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk815(t4536: f64, t4539: f64, t1214: f64, t5474: f64, t1213: f64, t5440: f64, t490: f64, t4310: f64, t4314: f64, t24: f64, t5285: f64, t1111: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15181 = t4536 * t4539;
    let t15200 = t5474 * t1214;
    let t15204 = t5440 * t1213;
    let t15205 = t490 * t15204;
    let t15225 = t4310 * t4314;
    let t15227 = t24 * t5285;
    let t15228 = t1111 * t15227;
    (t15181, t15200, t15204, t15205, t15225, t15227, t15228)
}
