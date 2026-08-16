//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1047/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1047(t13227: f64, t4554: f64, t3233: f64, t5026: f64, t1092: f64, t13202: f64, t13205: f64, t13208: f64, t13211: f64, t13214: f64, t13219: f64, t13222: f64, t13225: f64, t9522: f64) -> (f64, f64, f64) {
    let t13228 = t4554 * t13227;
    let t13230 = t5026 * t3233;
    let t13231 = t1092 * t13230;
    let t13234 = 0.11054629629629629629e-1_f64 * t13202 - 0.33163888888888888888e-2_f64 * t13205 + 0.66327777777777777776e-2_f64 * t13208 + 0.33163888888888888888e-2_f64 * t13211 + 0.16581944444444444444e-2_f64 * t13214 + 0.11054629629629629629e-2_f64 * t13219 - 0.58958024691358024689e-2_f64 * t13222 + 0.17687407407407407407e-1_f64 * t13225 - 0.14739506172839506172e-1_f64 * t13228 + 0.13265555555555555555e-1_f64 * t13231 + 0.11054629629629629629e-2_f64 * t9522;
    (t13228, t13231, t13234)
}
