//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1262/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1262<F: Float>(t24568: F, t56775: F, t7380: F, t24658: F, t55901: F, t894: F, t2601: F, t55906: F, t1: F, t16628: F, t16640: F, t16980: F, t24566: F, t24574: F, t2640: F, t2643: F, t2668: F, t2674: F, t2678: F, t313: F, t32008: F, t322: F, t3634: F, t41585: F, t4947: F, t49896: F, t49900: F, t51085: F, t51126: F, t56745: F, t862: F, t893: F) -> (F, F, F, F, F) {
    let t56897 = t56775 * t24568;
    let t56902 = t56775 * t7380;
    let t56908 = t894 * t24658 * t55901;
    let t56911 = t2601 * t55906;
    let t56931 = F::cast_from(0.6104852320306553446e1_f64) * t41585 + F::cast_from(0.94667510637550784468e-1_f64) * t2640 * t49900 * t4947 + F::cast_from(0.56296038352410615326e5_f64) * t24566 * t313 * t56897 * t1 - F::cast_from(0.84444057528615922988e5_f64) * t24574 * t313 * t56902 * t1 + F::cast_from(0.2951381987273961e-1_f64) * t893 * t56908 - t862 * t322 * t56911 / F::cast_from(48.0_f64) + t51085 / F::cast_from(36.0_f64) + F::cast_from(0.27471835441379490507e2_f64) * t2668 * t56745 * t2674 + F::cast_from(0.28977204965962526181e-1_f64) * t51126 + F::cast_from(0.5680050638253047068e0_f64) * t2640 * t3634 * t2643 * t16640 + F::cast_from(0.42074449172244793095e0_f64) * t2640 * t32008 * t2643 * t16628 - F::cast_from(0.18314556960919660338e2_f64) * t2678 * t49896 * t16980;
    (t56897, t56902, t56908, t56911, t56931)
}
