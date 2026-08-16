//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1138/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1138<F: Float>(t1159: F, t4524: F, t2824: F, t1123: F, t4501: F, t2851: F, t1129: F, t1297: F, t3663: F, t3662: F, t4544: F, t4512: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11266 = t1159 * t4524;
    let t11267 = t11266 * t2824;
    let t11270 = t4501 * t1123;
    let t11271 = t2851 * t11270;
    let t11274 = t4501 * t1129;
    let t11275 = t2851 * t11274;
    let t11278 = t3663 * t1297;
    let t11279 = t3662 * t11278;
    let t11282 = t1159 * t4544;
    let t11283 = t11282 * t2824;
    let t11288 = t4512 * t1123;
    (t11266, t11267, t11270, t11271, t11274, t11275, t11279, t11282, t11283, t11288)
}
