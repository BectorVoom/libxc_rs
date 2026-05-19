//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1189/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1189<F: Float>(t23573: F, t24391: F, t22015: F, t140: F, t329: F, t7312: F, t871: F, t1: F, t11018: F, t11020: F, t11025: F, t11029: F, t24498: F, t24556: F, t24559: F, t24566: F, t24569: F, t24574: F, t24575: F, t24580: F, t24584: F, t24594: F, t24596: F, t24601: F, t24611: F, t24613: F, t24615: F, t2640: F, t2641: F, t2644: F, t2678: F, t313: F, t322: F, t3814: F, t7453: F, t7456: F, t7835: F, t8114: F, t8134: F, t862: F, t878: F, t893: F) -> (F, F) {
    let t24619 = t24391 * t23573;
    let t24620 = t24619 * t22015;
    let t24626 = t329 * t7312 * t871 * t140;
    let t24629 = F::cast_from(0.21464596271083352727e-1_f64) * t24556 + F::cast_from(0.18110753103726578864e-2_f64) * t893 * t24559 + F::cast_from(0.56296038352410615326e5_f64) * t24566 * t313 * t24569 * t1 - F::cast_from(0.84444057528615922988e5_f64) * t24574 * t313 * t24575 * t1 + F::cast_from(0.19535527424980971027e3_f64) * t24580 * t7453 - F::cast_from(0.24419409281226213784e2_f64) * t24584 + F::cast_from(0.94667510637550784468e-1_f64) * t2640 * t2641 * t7835 * t2644 - F::cast_from(0.18314556960919660338e2_f64) * t2678 * t24498 * t3814 + F::cast_from(0.94667510637550784468e-1_f64) * t24594 - F::cast_from(0.17171677016866682182e-1_f64) * t24596 - t24601 + F::cast_from(0.63777043459628018514e5_f64) * t8134 * t7456 * t11025 - F::cast_from(0.63777043459628018516e5_f64) * t8114 * t7456 * t11029 + F::cast_from(0.1062950724327133642e5_f64) * t11018 * t7456 * t11020 - F::cast_from(0.39071054849961942054e3_f64) * t24611 + F::cast_from(0.19535527424980971027e3_f64) * t24613 + t862 * t322 * t24615 / F::new(288.0) + F::new(35.0) / F::new(972.0) * t862 * t322 * t24620 - F::cast_from(0.3517423950799664703e2_f64) * t24626 * t878;
    (t24620, t24629)
}
