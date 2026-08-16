//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2982/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2982<F: Float>(t16226: F, t16229: F, t53405: F, t3075: F, t4910: F, t1043: F, t43051: F, t3059: F, t4900: F, t3230: F, t4857: F, t11817: F, t4858: F) -> (F, F, F, F, F, F) {
    let t54348 = t16226 * t53405 * t16229;
    let t54360 = t4910 * t3075;
    let t54365 = t43051 * t1043;
    let t54370 = t4900 * t3059;
    let t54384 = t4857 * t3230;
    let t54387 = t4858 * t11817;
    (t54348, t54360, t54365, t54370, t54384, t54387)
}
