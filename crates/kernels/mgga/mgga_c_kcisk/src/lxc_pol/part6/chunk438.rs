//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 438/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk438<F: Float>(t1010: F, t3274: F, t224: F, t220: F, t967: F, t229: F, t1071: F, t142: F, t1070: F, t247: F, t242: F, t1077: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3275 = t1010 * t3274;
    let t3276 = t224 * t224;
    let t3277 = F::new(1.0) / t3276;
    let t3281 = t220 * t967;
    let t3288 = t229 * t229;
    let t3289 = F::new(1.0) / t3288;
    let t3306 = t142 * t1071;
    let t3310 = t1070 * t247;
    let t3311 = F::new(1.0) / t3310;
    let t3312 = t242 * t3311;
    let t3313 = t1077 * t1077;
    (t3275, t3276, t3277, t3281, t3288, t3289, t3306, t3311, t3312, t3313)
}
