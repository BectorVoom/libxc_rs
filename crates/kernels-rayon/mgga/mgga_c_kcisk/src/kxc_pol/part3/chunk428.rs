//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 428/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk428(t222: f64, t224: f64, t3277: f64, t3278: f64, t3283: f64, t229: f64, t1060: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t3287 = piecewise3(t223, 0.0_f64, 4.0_f64 / 9.0_f64 * t3277 * t3278 + 4.0_f64 / 3.0_f64 * t224 * t3283);
    let t3288 = t229 * t229;
    let t3289 = 1.0_f64 / t3288;
    let t3290 = t1060 * t1060;
    (t3287, t3288, t3289, t3290)
}
