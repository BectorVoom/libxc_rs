//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 669/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk669<F: Float>(t10336: F, t10337: F, t1006: F, t3185: F, t3187: F, t1053: F, t3274: F, t3186: F, t5060: F, t654: F, t140: F, t3737: F, t5180: F, sigma2: F) -> (F, F, F, F, F) {
    let t10338 = t10336 * t10337;
    let t10339 = F::new(6.0) * t10338;
    let t10340 = t1006 * t3185;
    let t10341 = t10340 * t3187;
    let t10342 = F::new(6.0) * t10341;
    let t10349 = t1053 * t3274;
    let t10350 = t3186 * t10349;
    let t10351 = F::new(6.0) * t10350;
    let t10364 = t5060 * sigma2;
    let t10365 = t10364 * t654;
    let t10409 = t140 * t3737 * t5180;
    (t10339, t10342, t10351, t10365, t10409)
}
