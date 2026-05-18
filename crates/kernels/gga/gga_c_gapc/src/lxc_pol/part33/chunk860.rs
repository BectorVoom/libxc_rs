//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 860/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk860<F: Float>(t3375: F, t9903: F, t3367: F, t3374: F, t3371: F, t2405: F, t2636: F, t3378: F, t8666: F, t916: F, t3384: F, t612: F, t7073: F) -> (F, F, F, F, F, F) {
    let t9997 = t9903 * t3375;
    let t9999 = t3367 * t3374;
    let t10000 = t3371 * t9999;
    let t10002 = t2636 * t2405;
    let t10003 = t3378 * t10002;
    let t10005 = t916 * t8666;
    let t10006 = t10005 * t3384;
    let t10008 = t7073 * t612;
    (t9997, t9999, t10000, t10003, t10006, t10008)
}
