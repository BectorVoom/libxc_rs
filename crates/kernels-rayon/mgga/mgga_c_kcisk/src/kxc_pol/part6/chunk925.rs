//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 925/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk925(t28385: f64, t7378: f64, t28377: f64, t7370: f64, t18036: f64, t5248: f64, t7715: f64, t11910: f64, t11911: f64, t28368: f64, t1470: f64, t18005: f64, t24299: f64, t24320: f64, t24324: f64, t24374: f64, t24376: f64, t2543: f64, t28859: f64, t28868: f64, t28873: f64, t28885: f64, t6278: f64, t725: f64, t8915: f64, t8923: f64, t8927: f64, t8931: f64) -> f64 {
    let t29404 = t7378 * t28385;
    let t29416 = t7370 * t28377;
    let t29432 = t5248 * t18036 * t7715;
    let t29436 = t11910 * t11911 * t28368;
    let t29439 = 0.26531111111111111111e-1_f64 * t18005 + 0.15918666666666666666e0_f64 * t6278 * t29404 - 0.10612444444444444444e0_f64 * t24299 + 0.371475e-1_f64 * t2543 * t8927 - 0.232171875e-2_f64 * t725 * t28885 + 0.139303125e-1_f64 * t2543 * t8915 - 0.88437037037037037035e-1_f64 * t24320 - 0.79593333333333333333e-1_f64 * t24324 - 0.13265555555555555555e0_f64 * t6278 * t29416 - 0.619125e-2_f64 * t725 * t28868 + 0.27860625e-1_f64 * t2543 * t8923 - 0.1857375e-1_f64 * t2543 * t8931 + 0.9286875e-2_f64 * t725 * t28873 + 0.10612444444444444444e0_f64 * t24374 - 0.53062222222222222221e-1_f64 * t24376 - 0.371475e-1_f64 * t725 * t28859 - 0.13265555555555555556e0_f64 * t1470 * t29432 - 0.11791604938271604938e0_f64 * t1470 * t29436;
    t29439
}
