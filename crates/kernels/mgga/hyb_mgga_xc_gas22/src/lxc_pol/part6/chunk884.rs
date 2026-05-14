//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 884/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk884<F: Float>(t3014: F, t545: F, t7969: F, t1796: F, t3015: F, t39: F, t574: F, t577: F, t3023: F, t35: F, t572: F, t6007: F, t6010: F, t6013: F, t6015: F, t6017: F, t6019: F, t7933: F, t7936: F, t7938: F, t7943: F, t7948: F, t7953: F, t7958: F, t7962: F, t7966: F) -> (F, F, F, F, F) {
    let t7971 = t3014 * t7969 * t545;
    let t7975 = t3014 * t3015 * t1796;
    let t7978 = t574 * t39;
    let t7979 = t7978 * t577;
    let t7983 = -t6010 - 4.0 / 243.0 * t6013 + t6015 / 243.0 - t6017 / 81.0 + t6019 / 162.0 - 2.0 / 243.0 * t7933 + t7936 - t7938 + 11.0 / 81.0 * t7943 - 5.0 / 243.0 * t572 * t7948 + 2.0 / 27.0 * t572 * t7953 - 4.0 / 81.0 * t3023 * t7958 - t572 * t7962 / 81.0 - t572 * t7966 / 9.0 + 4.0 / 27.0 * t3023 * t7971 + t572 * t7975 / 27.0 - t35 * t6007 * t7979 / 27.0;
    (t7971, t7975, t7978, t7979, t7983)
}
