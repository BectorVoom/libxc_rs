//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1108/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1108<F: Float>(t2473: F, t4270: F, t4247: F, t7009: F, t952: F, t3485: F, t3490: F, t2484: F, t4251: F, t222: F, t4234: F, t568: F) -> (F, F, F, F, F, F, F) {
    let t10886 = F::new(1.0) * t2473 * t4270;
    let t10887 = t7009 * t4247;
    let t10888 = t10887 * t952;
    let t10890 = t3485 * t3490;
    let t10892 = t2484 * t4251;
    let t10893 = t10892 * t952;
    let t10898 = t222 * t568 * t4234;
    (t10886, t10887, t10888, t10890, t10892, t10893, t10898)
}
