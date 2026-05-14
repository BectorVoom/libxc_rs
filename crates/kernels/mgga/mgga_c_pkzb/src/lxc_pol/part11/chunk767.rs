//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 767/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk767<F: Float>(t7930: F, t7979: F, t7982: F, t1201: F, t2295: F, t3113: F, t881: F, t2317: F, t3080: F, t862: F, t1189: F, t2278: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8038 = 0.18541666666666666667e-1 * t7930;
    let t8045 = 0.103295e1 * t7930;
    let t8059 = 0.41678e0 * t7979;
    let t8060 = 0.41678e0 * t7982;
    let t8071 = t1201 * t2295;
    let t8076 = 0.60385e0 * t7930;
    let t8090 = 0.33114e0 * t7979;
    let t8091 = 0.33114e0 * t7982;
    let t8102 = t3113 * t881;
    let t8107 = t1201 * t2317;
    let t8115 = t3080 * t862;
    let t8120 = t1189 * t2278;
    (t8038, t8045, t8059, t8060, t8071, t8076, t8090, t8091, t8102, t8107, t8115, t8120)
}
