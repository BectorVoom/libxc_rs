//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 974/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk974(t1005: f64, t5089: f64, t13635: f64, t527: f64, t3371: f64, t4198: f64, t4452: f64, t4384: f64, t4389: f64, t12813: f64, t4967: f64, t13084: f64, t4971: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15891 = t1005 * t5089;
    let t15902 = t13635 * t527;
    let t15905 = t4198 * t3371;
    let t15906 = t15905 * t4452;
    let t15914 = t4389 * t4384;
    let t15916 = t12813 * t4967;
    let t15918 = t13084 * t4971;
    (t15891, t15902, t15905, t15906, t15914, t15916, t15918)
}
