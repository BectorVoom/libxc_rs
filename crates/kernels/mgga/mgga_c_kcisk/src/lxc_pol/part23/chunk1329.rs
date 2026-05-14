//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1329/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1329<F: Float>(t14287: F, t33636: F, t109321: F, t20931: F, t33633: F, t4170: F, t4321: F, t9848: F, t21044: F, t32278: F, t32277: F, t5885: F, t4232: F, t21085: F, t19715: F, t6332: F, t9491: F) -> (F, F, F, F, F, F, F, F) {
    let t113320 = 4.0 * t14287 * t33636;
    let t113322 = 6.0 * t109321 * t20931;
    let t113324 = 4.0 * t14287 * t33633;
    let t113347 = 2.0 * t4170 * t9848 * t4321;
    let t113348 = t32278 * t21044;
    let t113350 = t5885 * t32277;
    let t113351 = t113350 * t4232;
    let t113353 = t32278 * t21085;
    let t113356 = t9491 * t6332 * t19715;
    (t113320, t113322, t113324, t113347, t113348, t113351, t113353, t113356)
}
