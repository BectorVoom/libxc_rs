//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1262/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1262(t24568: f64, t56775: f64, t7380: f64, t24658: f64, t55901: f64, t894: f64, t2601: f64, t55906: f64, t1: f64, t16628: f64, t16640: f64, t16980: f64, t24566: f64, t24574: f64, t2640: f64, t2643: f64, t2668: f64, t2674: f64, t2678: f64, t313: f64, t32008: f64, t322: f64, t3634: f64, t41585: f64, t4947: f64, t49896: f64, t49900: f64, t51085: f64, t51126: f64, t56745: f64, t862: f64, t893: f64) -> (f64, f64, f64, f64, f64) {
    let t56897 = t56775 * t24568;
    let t56902 = t56775 * t7380;
    let t56908 = t894 * t24658 * t55901;
    let t56911 = t2601 * t55906;
    let t56931 = 0.6104852320306553446e1_f64 * t41585 + 0.94667510637550784468e-1_f64 * t2640 * t49900 * t4947 + 0.56296038352410615326e5_f64 * t24566 * t313 * t56897 * t1 - 0.84444057528615922988e5_f64 * t24574 * t313 * t56902 * t1 + 0.2951381987273961e-1_f64 * t893 * t56908 - t862 * t322 * t56911 / 48.0_f64 + t51085 / 36.0_f64 + 0.27471835441379490507e2_f64 * t2668 * t56745 * t2674 + 0.28977204965962526181e-1_f64 * t51126 + 0.5680050638253047068e0_f64 * t2640 * t3634 * t2643 * t16640 + 0.42074449172244793095e0_f64 * t2640 * t32008 * t2643 * t16628 - 0.18314556960919660338e2_f64 * t2678 * t49896 * t16980;
    (t56897, t56902, t56908, t56911, t56931)
}
