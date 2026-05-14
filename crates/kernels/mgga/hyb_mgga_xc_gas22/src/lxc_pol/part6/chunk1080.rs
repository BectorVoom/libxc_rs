//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1080/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1080<F: Float>(t1123: F, t4501: F, t2851: F, t1129: F, t1297: F, t3663: F, t3662: F, t1159: F, t4544: F, t2824: F, t4512: F, t1161: F, t2858: F, t11267: F, t2821: F, t2829: F, t2834: F, t2838: F, t3661: F, t3680: F, t3688: F, t3733: F, t7637: F) -> (F, F, F, F, F, F) {
    let t11270 = t4501 * t1123;
    let t11271 = t2851 * t11270;
    let t11274 = t4501 * t1129;
    let t11275 = t2851 * t11274;
    let t11278 = t3663 * t1297;
    let t11279 = t3662 * t11278;
    let t11282 = t1159 * t4544;
    let t11283 = t11282 * t2824;
    let t11288 = t4512 * t1123;
    let t11289 = t1161 * t11288;
    let t11292 = t4512 * t1129;
    let t11293 = t1161 * t11292;
    let t11296 = t2858 * t11270;
    let t11299 = t2858 * t11274;
    let t11310 = -56.0 / 3.0 * t7637 * t11267 - 64.0 / 81.0 * t3733 * t11271 + 64.0 / 81.0 * t3661 * t11275 + 400.0 / 27.0 * t3733 * t11279 + 8.0 / 9.0 * t2829 * t11283 + 400.0 / 27.0 * t3661 * t11279 + 88.0 / 27.0 * t2821 * t11289 - 88.0 / 27.0 * t2829 * t11293 - 32.0 / 27.0 * t2821 * t11296 + 32.0 / 27.0 * t2829 * t11299 - 64.0 / 27.0 * t3680 * t11271 + 64.0 / 27.0 * t3688 * t11275 - 32.0 / 9.0 * t2834 * t11296 + 32.0 / 9.0 * t2838 * t11299;
    (t11279, t11282, t11283, t11289, t11293, t11310)
}
