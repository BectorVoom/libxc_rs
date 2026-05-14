//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 779/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk779<F: Float>(t3371: F, t9999: F, t2405: F, t2636: F, t3378: F, t8666: F, t916: F, t3384: F, t612: F, t7073: F, t2545: F, t7200: F, t320: F, t9029: F, t315: F, t7216: F) -> (F, F, F, F, F, F) {
    let t10000 = t3371 * t9999;
    let t10002 = t2636 * t2405;
    let t10003 = t3378 * t10002;
    let t10005 = t916 * t8666;
    let t10006 = t10005 * t3384;
    let t10008 = t7073 * t612;
    let t10009 = t2545 * t7200;
    let t10010 = t10008 * t10009;
    let t10012 = t320 * t9029;
    let t10013 = t315 * t7216;
    (t10000, t10003, t10006, t10010, t10012, t10013)
}
