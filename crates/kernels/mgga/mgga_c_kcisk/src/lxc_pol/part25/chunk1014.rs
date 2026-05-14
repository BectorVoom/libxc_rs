//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1014/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1014<F: Float>(t17930: F, t5321: F, t5320: F, t6973: F, t5323: F, t718: F, t7336: F, t5291: F, t11807: F, t79: F, t17132: F, t7310: F, t11717: F, t7440: F, t17126: F, t5290: F) -> (F, F, F, F, F, F, F) {
    let t17931 = t5321 * t17930;
    let t17933 = t6973 * t5320;
    let t17934 = t17933 * t5323;
    let t17936 = t7336 * t718;
    let t17937 = t17936 * t5291;
    let t17939 = t79 * t11807;
    let t17940 = t17939 * t17132;
    let t17941 = t7310 * t17940;
    let t17943 = t11717 * t7440;
    let t17945 = t5290 * t17126;
    (t17931, t17934, t17937, t17940, t17941, t17943, t17945)
}
