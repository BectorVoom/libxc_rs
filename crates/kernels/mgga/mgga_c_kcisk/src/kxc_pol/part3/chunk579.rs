//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 579/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk579<F: Float>(t4816: F, t740: F, t1950: F, t1945: F, t1954: F, t5061: F, t5063: F, t747: F, t746: F, t745: F, t1872: F, t641: F, t79: F, t5068: F, t4797: F, t1948: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5310 = t4816 * t740;
    let t5311 = t5310 * t1950;
    let t5313 = t1945 * t1954;
    let t5315 = t5061 * t740;
    let t5316 = t747 * t5063;
    let t5317 = t746 * t5316;
    let t5318 = t5315 * t5317;
    let t5320 = t740 * t745;
    let t5321 = t1872 * t5320;
    let t5322 = t79 * t641;
    let t5323 = t5322 * t5068;
    let t5324 = t5321 * t5323;
    let t5326 = t747 * t4797;
    let t5327 = t746 * t5326;
    let t5328 = t1948 * t5327;
    (t5310, t5311, t5313, t5317, t5318, t5320, t5321, t5322, t5323, t5324, t5327, t5328)
}
