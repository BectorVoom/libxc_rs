//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 958/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk958(t5368: f64, t852: f64, t119: f64, t5299: f64, t5360: f64, t880: f64, t3054: f64, t545: f64, t865: f64, t5332: f64, t857: f64, t3874: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15278 = t852 * t5368;
    let t15285 = t119 * t5299;
    let t15290 = t5360 * t880;
    let t15293 = t3054 * t545 * t865;
    let t15295 = t857 * t5332;
    let t15297 = t3874 * t556;
    (t15278, t15285, t15290, t15293, t15295, t15297)
}
