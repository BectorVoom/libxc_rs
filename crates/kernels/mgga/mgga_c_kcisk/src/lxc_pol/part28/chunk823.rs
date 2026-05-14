//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 823/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk823<F: Float>(t10043: F, t567: F, t1008: F, t195: F, t1053: F, t3187: F, t1006: F, t3185: F, t3274: F, t5060: F, t654: F, t140: F, t3737: F, t5180: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10044 = t567 * t10043;
    let t10334 = t1008 * t1008;
    let t10335 = 1.0 / t10334;
    let t10336 = t195 * t10335;
    let t10337 = t3187 * t1053;
    let t10340 = t1006 * t3185;
    let t10349 = t1053 * t3274;
    let t10364 = t5060 * sigma2;
    let t10365 = t10364 * t654;
    let t10409 = t140 * t3737 * t5180;
    (t10044, t10334, t10335, t10336, t10337, t10340, t10349, t10364, t10365, t10409)
}
