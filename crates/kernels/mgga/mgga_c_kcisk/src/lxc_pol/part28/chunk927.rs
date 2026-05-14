//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 927/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk927<F: Float>(t172: F, t41: F, t139: F, t2543: F, t574: F, t4265: F, t7391: F, t2551: F, t979: F, t6941: F, t695: F, t7375: F, t15197: F, t7383: F, t2041: F, t7654: F) -> (F, F, F, F, F, F, F, F) {
    let t18080 = t172 * t41;
    let t18081 = t139 * t18080;
    let t18089 = t2543 * t574;
    let t18092 = t4265 * t7391;
    let t18132 = t979 * t2551;
    let t18147 = t6941 * t695;
    let t18155 = 0.35374814814814814814e-1 * t4265 * t7375;
    let t18156 = t15197 * t7383;
    let t18179 = t7654 * t2041;
    (t18081, t18089, t18092, t18132, t18147, t18155, t18156, t18179)
}
