//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 912/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk912<F: Float>(t11287: F, t3659: F, t4908: F, t687: F, t4915: F, t1049: F, t3179: F, t1616: F, t1611: F, t3721: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11288 = F::new(2.0) * t11287;
    let t11289 = t4908 * t3659;
    let t11290 = F::new(2.0) * t11289;
    let t11291 = t3659 * t687;
    let t11292 = t4915 * t11291;
    let t11293 = F::new(6.0) * t11292;
    let t11294 = t1049 * t3179;
    let t11295 = t1616 * t11294;
    let t11296 = F::new(4.0) * t11295;
    let t11297 = t1611 * t3721;
    let t11298 = t3721 * t687;
    let t11299 = t1616 * t11298;
    (t11288, t11289, t11290, t11291, t11292, t11293, t11294, t11295, t11296, t11297, t11298, t11299)
}
