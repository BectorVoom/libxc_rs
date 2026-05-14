//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 960/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk960<F: Float>(t13917: F, t8032: F, t1309: F, t3959: F, t8054: F, t1322: F, t6204: F, t1233: F, t7922: F, t1248: F, t25446: F, t4065: F, t13607: F, t25465: F, t20448: F, t25469: F) -> (F, F, F, F, F, F) {
    let t26085 = t13917 * t8032;
    let t26086 = t1309 * t26085;
    let t26088 = t3959 * t8054;
    let t26089 = t26088 * t1322;
    let t26090 = t6204 * t26089;
    let t26095 = t7922 * t1233;
    let t26110 = t1248 * t4065 * t25446;
    let t26113 = t1248 * t13607 * t25465;
    let t26116 = t1248 * t20448 * t25469;
    (t26086, t26090, t26095, t26110, t26113, t26116)
}
