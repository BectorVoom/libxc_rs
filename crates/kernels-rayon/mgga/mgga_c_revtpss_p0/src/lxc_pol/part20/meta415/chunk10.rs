//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1545/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1545(t12153: f64, t3046: f64, t12046: f64, t989: f64, t1035: f64, t42859: f64, t342: f64, t357: f64, t43351: f64, t1024: f64, t1043: f64, t1087: f64, t1089: f64, t11173: f64, t12032: f64, t12053: f64, t12073: f64, t12097: f64, t12100: f64, t12122: f64, t12127: f64, t3075: f64, t3223: f64, t3283: f64, t3288: f64, t3291: f64, t3304: f64, t3309: f64, t3318: f64, t42359: f64, t42894: f64, t43348: f64) -> f64 {
    let t43378 = t3046 * t12153;
    let t43384 = t989 * t12046;
    let t43400 = t42859 * t1035;
    let t43401 = t342 * t43400;
    let t43402 = t43351 * t357;
    let t43409 = 0.39512695097613069592e1_f64 * t12127 * t42894 * t3318 - 0.15805078039045227836e2_f64 * t43378 * t3288 - 0.79025390195226139183e1_f64 * t12122 * t42894 * t3304 + 0.26341796731742046395e1_f64 * t43384 * t12053 - 0.79025390195226139183e1_f64 * t3223 * t12100 + 0.79025390195226139183e1_f64 * t42359 * t3283 + 0.79025390195226139183e1_f64 * t12097 * t3309 + 0.26341796731742046395e1_f64 * t1087 * t12032 * t1043 * t1089 - 0.39512695097613069592e1_f64 * t1024 * t12073 * t3075 - 0.65854491829355115987e0_f64 * t43401 * t43348 * t43402 - 0.26341796731742046395e1_f64 * t1024 * t3291 * t11173;
    t43409
}
