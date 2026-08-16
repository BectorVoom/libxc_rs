//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 645/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk645(t10352: f64, t1152: f64, t3474: f64, t1636: f64, t5294: f64, t5184: f64, t5182: f64, t5302: f64, t5192: f64, t5060: f64, t654: f64, t5285: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10353 = 3.0_f64 / 16.0_f64 * t10352;
    let t10354 = t1152 * t3474;
    let t10355 = 3.0_f64 / 16.0_f64 * t10354;
    let t10356 = t5294 * t1636;
    let t10357 = t5184 * t10356;
    let t10358 = t5182 * t10357;
    let t10360 = t5302 * t1636;
    let t10361 = t5192 * t10360;
    let t10362 = t5182 * t10361;
    let t10364 = t5060 * sigma2;
    let t10365 = t10364 * t654;
    let t10366 = t5285 * t1636;
    (t10353, t10355, t10358, t10362, t10365, t10366)
}
