//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1187/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1187(t1111: f64, t17903: f64, t24: f64, t17907: f64, t17723: f64, t2586: f64, t1133: f64, t4356: f64, t5110: f64, t15327: f64, t4380: f64, t17922: f64, t27031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54295 = t1111 * t24 * t17903;
    let t54298 = t1111 * t24 * t17907;
    let t54304 = t2586 * t17723;
    let t54305 = t1133 * t54304;
    let t54308 = t4356 * t5110;
    let t54317 = t15327 * t4380;
    let t54341 = t27031 * t17922;
    (t54295, t54298, t54304, t54305, t54308, t54317, t54341)
}
