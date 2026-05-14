//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1314/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1314<F: Float>(t16569: F, t2789: F, t415: F, t7233: F, t9650: F, t33033: F, t6758: F, t17357: F, t33031: F, t34017: F, t33003: F, t5014: F, t2469: F, t695: F, t112192: F, t2364: F, t32936: F) -> (F, F, F, F, F, F) {
    let t116912 = t415 * t16569 * t2789;
    let t116914 = t7233 * t9650;
    let t116916 = t116914 * t6758 * t33033;
    let t116921 = 0.30864197530864197531e-2 * t33031 * t17357 * t34017;
    let t116922 = t5014 * t33003;
    let t116923 = t2469 * t695;
    let t116925 = t116922 * t116923 * t33033;
    let t116929 = t112192 * t2364 * t32936;
    (t116912, t116916, t116921, t116923, t116925, t116929)
}
