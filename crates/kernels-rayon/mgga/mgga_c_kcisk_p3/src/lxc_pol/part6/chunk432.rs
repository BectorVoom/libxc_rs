//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 432/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk432(t218: f64, t3179: f64, t1006: f64, t1009: f64, t1053: f64, t1008: f64, t217: f64, t195: f64, t196: f64, t2925: f64, t179: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3180 = t3179 * t218;
    let t3181 = t1006 * t1009;
    let t3182 = t3181 * t1053;
    let t3183 = 2.0_f64 * t3182;
    let t3185 = 1.0_f64 / t1008 / t217;
    let t3186 = t195 * t3185;
    let t3187 = t1053 * t1053;
    let t3188 = t3186 * t3187;
    let t3189 = 2.0_f64 * t3188;
    let t3190 = t2925 * t196;
    let t3193 = t852 * t179;
    (t3180, t3181, t3182, t3183, t3185, t3186, t3187, t3188, t3189, t3190, t3193)
}
