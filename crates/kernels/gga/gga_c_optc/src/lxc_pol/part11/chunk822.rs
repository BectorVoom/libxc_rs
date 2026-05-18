//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 822/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk822<F: Float>(t15622: F, t3103: F, t140: F, t1514: F, t309: F, t4383: F, t1: F, t12002: F, t2855: F, t438: F, t1129: F, t5280: F) -> (F, F, F, F, F) {
    let t15623 = t3103 * t15622;
    let t15641 = t1514 * t309 * t140;
    let t15642 = t4383 * t15641;
    let t15653 = t12002 * t1;
    let t15654 = t438 * t2855;
    let t15660 = t5280 * t1129;
    (t15623, t15642, t15653, t15654, t15660)
}
