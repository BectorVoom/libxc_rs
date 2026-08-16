//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2982/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2982(t16226: f64, t16229: f64, t53405: f64, t3075: f64, t4910: f64, t1043: f64, t43051: f64, t3059: f64, t4900: f64, t3230: f64, t4857: f64, t11817: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54348 = t16226 * t53405 * t16229;
    let t54360 = t4910 * t3075;
    let t54365 = t43051 * t1043;
    let t54370 = t4900 * t3059;
    let t54384 = t4857 * t3230;
    let t54387 = t4858 * t11817;
    (t54348, t54360, t54365, t54370, t54384, t54387)
}
