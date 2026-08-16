//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1033/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1033(t60: f64, t12630: f64, t123: f64, t925: f64, t3015: f64, t896: f64, t2994: f64, t3006: f64, t898: f64, t2995: f64, t3012: f64, t3: f64, t74: f64, t83: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t124 = 0.0_f64 < t60;
    let t15268 = piecewise3(t124, t12630, -t12630);
    let t15270 = t123 * t925 * t15268;
    let t15274 = t3015 * t896;
    let t15278 = t2994 * t896;
    let t15279 = t898 * t3006;
    let t15283 = t2995 * t896;
    let t15285 = t3012 * t15283 * t898;
    let t15291 = 1.0_f64 / t74 / t83 * t3 / 4.0_f64;
    (t15270, t15274, t15278, t15279, t15283, t15285, t15291)
}
