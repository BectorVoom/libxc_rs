//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 621/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk621<F: Float>(t1001: F, t167: F, t2689: F, t1049: F, t116: F, t3182: F, t1008: F, t195: F, t1053: F, t3187: F, t1006: F, t3185: F, t3274: F, t3186: F, t5060: F, t654: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t9345 = t167 * t1001;
    let t9352 = t2689 * t1001;
    let t9355 = t116 * t1049;
    let t10328 = 6.0 * t3182;
    let t10334 = t1008 * t1008;
    let t10335 = 1.0 / t10334;
    let t10336 = t195 * t10335;
    let t10337 = t3187 * t1053;
    let t10338 = t10336 * t10337;
    let t10339 = 6.0 * t10338;
    let t10340 = t1006 * t3185;
    let t10341 = t10340 * t3187;
    let t10342 = 6.0 * t10341;
    let t10349 = t1053 * t3274;
    let t10350 = t3186 * t10349;
    let t10351 = 6.0 * t10350;
    let t10364 = t5060 * sigma2;
    let t10365 = t10364 * t654;
    (t9345, t9352, t9355, t10328, t10339, t10342, t10351, t10365)
}
