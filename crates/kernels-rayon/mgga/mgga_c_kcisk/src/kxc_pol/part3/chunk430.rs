//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 430/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk430(t227: f64, t229: f64, t3289: f64, t3290: f64, t3293: f64, t3287: f64, t44: f64, t291: f64, t1065: f64, t1149: f64, t1071: f64, t142: f64, t1070: f64, t247: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t3297 = piecewise3(t228, 0.0_f64, 4.0_f64 / 9.0_f64 * t3289 * t3290 + 4.0_f64 / 3.0_f64 * t229 * t3293);
    let t3299 = (t3287 + t3297) * t44;
    let t3300 = t3299 * t291;
    let t3301 = t1065 * t1149;
    let t3306 = t142 * t1071;
    let t3310 = t1070 * t247;
    (t3299, t3300, t3301, t3306, t3310)
}
