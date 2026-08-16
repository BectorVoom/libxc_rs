//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 865/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk865<F: Float>(t2923: F, t9282: F, t302: F, t3542: F, t759: F, t761: F, t5693: F, t3645: F, t7701: F, t7700: F, t2003: F, t655: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9283 = t9282 * t2923;
    let t9284 = t302 * t9283;
    let t9287 = t3542 * t759;
    let t9288 = t9287 * t761;
    let t9289 = t5693 * t9288;
    let t9292 = t7701 * t3645;
    let t9293 = t7700 * t9292;
    let t9296 = t2003 * t3542;
    let t9297 = t9296 * t655;
    (t9283, t9284, t9287, t9288, t9289, t9292, t9293, t9296, t9297)
}
