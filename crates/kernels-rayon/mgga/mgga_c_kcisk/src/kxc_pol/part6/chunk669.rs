//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 669/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk669(t10336: f64, t10337: f64, t1006: f64, t3185: f64, t3187: f64, t1053: f64, t3274: f64, t3186: f64, t5060: f64, t654: f64, t140: f64, t3737: f64, t5180: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t10338 = t10336 * t10337;
    let t10339 = 6.0_f64 * t10338;
    let t10340 = t1006 * t3185;
    let t10341 = t10340 * t3187;
    let t10342 = 6.0_f64 * t10341;
    let t10349 = t1053 * t3274;
    let t10350 = t3186 * t10349;
    let t10351 = 6.0_f64 * t10350;
    let t10364 = t5060 * sigma2;
    let t10365 = t10364 * t654;
    let t10409 = t140 * t3737 * t5180;
    (t10339, t10342, t10351, t10365, t10409)
}
