//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 908/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk908<F: Float>(t1796: F, t3008: F, t3009: F, t1808: F, t3014: F, t1802: F, t3: F, t545: F, t3015: F, t39: F, t574: F, t577: F) -> (F, F, F, F, F, F) {
    let t7962 = t3008 * t3009 * t1796;
    let t7966 = t3014 * t3009 * t1808;
    let t7969 = t1802 * t3;
    let t7971 = t3014 * t7969 * t545;
    let t7975 = t3014 * t3015 * t1796;
    let t7978 = t574 * t39;
    let t7979 = t7978 * t577;
    (t7962, t7966, t7971, t7975, t7978, t7979)
}
