//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 420/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk420<F: Float>(t227: F, t229: F, t3289: F, t3290: F, t3293: F, t3287: F, t44: F, t291: F, t1065: F, t1149: F, t1071: F, t142: F, t1070: F, t247: F, t242: F, t1077: F, t1078: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t3297 = piecewise3(t228, 0.0, 4.0 / 9.0 * t3289 * t3290 + 4.0 / 3.0 * t229 * t3293);
    let t3299 = (t3287 + t3297) * t44;
    let t3300 = t3299 * t291;
    let t3301 = t1065 * t1149;
    let t3306 = t142 * t1071;
    let t3310 = t1070 * t247;
    let t3311 = 1.0 / t3310;
    let t3312 = t242 * t3311;
    let t3313 = t1077 * t1077;
    let t3314 = t3313 * t1078;
    (t3299, t3300, t3301, t3306, t3311, t3312, t3313, t3314)
}
