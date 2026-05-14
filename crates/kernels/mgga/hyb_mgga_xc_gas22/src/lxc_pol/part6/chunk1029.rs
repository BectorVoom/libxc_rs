//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1029/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1029<F: Float>(t10350: F, t10364: F, t10373: F, t10424: F, t1270: F, t1282: F, t172: F, t184: F, t2116: F, t3227: F, t3231: F, t3246: F, t3264: F, t4046: F, t4051: F, t4068: F, t4079: F, t6363: F, t740: F, t742: F, t756: F, t8373: F, t8395: F) -> (F,) {
    let t10427 = 7.0 / 2.0 * t4068 * t3246 - t8395 * t8373 - t10364 * t3246 / 4.0 - 6.0 * t6363 * t4051 * t740 + 4.0 * t2116 * t1270 * t3227 - t3231 * t10373 / 2.0 + 2.0 * t2116 * t4046 * t740 - t742 * t10350 + 2.0 * t10350 * t184 + 2.0 * t4046 * t756 + 4.0 * t3227 * t1282 + 4.0 * t1270 * t3264 + 2.0 * t740 * t4079 + 2.0 * t172 * t10424;
    (t10427,)
}
