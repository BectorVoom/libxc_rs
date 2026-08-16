//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1189/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1189(t17727: f64, t45811: f64, t1129: f64, t17886: f64, t15696: f64, t4310: f64, t12068: f64, t17344: f64, t4386: f64, t15227: f64, t15693: f64, t15321: f64, t4369: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54451 = t45811 * t17727;
    let t54470 = t17886 * t1129;
    let t54472 = t4310 * t15696;
    let t54477 = t4386 * t12068 * t17344;
    let t54509 = t4310 * t15227;
    let t54511 = t4310 * t15693;
    let t54518 = t4369 * t15321;
    (t54451, t54470, t54472, t54477, t54509, t54511, t54518)
}
