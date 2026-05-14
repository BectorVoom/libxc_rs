//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1280/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1280<F: Float>(t10473: F, t9946: F, t32995: F, t34125: F, t112176: F, t1869: F, t34221: F, t32909: F, t34225: F, t17141: F, t5054: F, t9679: F, t16623: F, t33017: F, t6713: F, t5014: F, t9650: F) -> (F, F, F, F, F, F, F) {
    let t116167 = t10473 * t9946;
    let t116170 = 0.18518518518518518519e-1 * t34125 * t32995;
    let t116174 = t1869 * t112176 * t34221;
    let t116176 = t34225 * t32909;
    let t116181 = t5054 * t9679 * t17141;
    let t116184 = t6713 * t33017 * t16623;
    let t116186 = t5014 * t9650;
    (t116167, t116170, t116174, t116176, t116181, t116184, t116186)
}
