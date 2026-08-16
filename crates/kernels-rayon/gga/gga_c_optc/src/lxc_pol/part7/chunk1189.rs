//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1189/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1189(t23573: f64, t24391: f64, t22015: f64, t140: f64, t329: f64, t7312: f64, t871: f64, t1: f64, t11018: f64, t11020: f64, t11025: f64, t11029: f64, t24498: f64, t24556: f64, t24559: f64, t24566: f64, t24569: f64, t24574: f64, t24575: f64, t24580: f64, t24584: f64, t24594: f64, t24596: f64, t24601: f64, t24611: f64, t24613: f64, t24615: f64, t2640: f64, t2641: f64, t2644: f64, t2678: f64, t313: f64, t322: f64, t3814: f64, t7453: f64, t7456: f64, t7835: f64, t8114: f64, t8134: f64, t862: f64, t878: f64, t893: f64) -> (f64, f64) {
    let t24619 = t24391 * t23573;
    let t24620 = t24619 * t22015;
    let t24626 = t329 * t7312 * t871 * t140;
    let t24629 = 0.21464596271083352727e-1_f64 * t24556 + 0.18110753103726578864e-2_f64 * t893 * t24559 + 0.56296038352410615326e5_f64 * t24566 * t313 * t24569 * t1 - 0.84444057528615922988e5_f64 * t24574 * t313 * t24575 * t1 + 0.19535527424980971027e3_f64 * t24580 * t7453 - 0.24419409281226213784e2_f64 * t24584 + 0.94667510637550784468e-1_f64 * t2640 * t2641 * t7835 * t2644 - 0.18314556960919660338e2_f64 * t2678 * t24498 * t3814 + 0.94667510637550784468e-1_f64 * t24594 - 0.17171677016866682182e-1_f64 * t24596 - t24601 + 0.63777043459628018514e5_f64 * t8134 * t7456 * t11025 - 0.63777043459628018516e5_f64 * t8114 * t7456 * t11029 + 0.1062950724327133642e5_f64 * t11018 * t7456 * t11020 - 0.39071054849961942054e3_f64 * t24611 + 0.19535527424980971027e3_f64 * t24613 + t862 * t322 * t24615 / 288.0_f64 + 35.0_f64 / 972.0_f64 * t862 * t322 * t24620 - 0.3517423950799664703e2_f64 * t24626 * t878;
    (t24620, t24629)
}
