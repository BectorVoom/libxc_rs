//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 809/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk809<F: Float>(t555: F, t560: F, t6160: F, t1861: F, t92: F, t10: F, t6033: F, t1802: F, t28: F, t125: F, t1994: F, t639: F, t668: F, t1783: F, t1819: F, t1788: F) -> (F, F, F, F, F, F, F, F) {
    let t6162 = t555 * t6160 * t560;
    let t6164 = t92 * t1861;
    let t6181 = t6033 * t10;
    let t6184 = 1.0 / t28 / t1802;
    let t6190 = t1994 * t125;
    let t6195 = t639 * t668;
    let t6201 = t555 * t1819 * t1783;
    let t6204 = t555 * t1819 * t1788;
    (t6162, t6164, t6181, t6184, t6190, t6195, t6201, t6204)
}
