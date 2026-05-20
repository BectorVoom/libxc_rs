//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1545/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1545<F: Float>(t12153: F, t3046: F, t12046: F, t989: F, t1035: F, t42859: F, t342: F, t357: F, t43351: F, t1024: F, t1043: F, t1087: F, t1089: F, t11173: F, t12032: F, t12053: F, t12073: F, t12097: F, t12100: F, t12122: F, t12127: F, t3075: F, t3223: F, t3283: F, t3288: F, t3291: F, t3304: F, t3309: F, t3318: F, t42359: F, t42894: F, t43348: F) -> F {
    let t43378 = t3046 * t12153;
    let t43384 = t989 * t12046;
    let t43400 = t42859 * t1035;
    let t43401 = t342 * t43400;
    let t43402 = t43351 * t357;
    let t43409 = F::cast_from(0.39512695097613069592e1_f64) * t12127 * t42894 * t3318 - F::cast_from(0.15805078039045227836e2_f64) * t43378 * t3288 - F::cast_from(0.79025390195226139183e1_f64) * t12122 * t42894 * t3304 + F::cast_from(0.26341796731742046395e1_f64) * t43384 * t12053 - F::cast_from(0.79025390195226139183e1_f64) * t3223 * t12100 + F::cast_from(0.79025390195226139183e1_f64) * t42359 * t3283 + F::cast_from(0.79025390195226139183e1_f64) * t12097 * t3309 + F::cast_from(0.26341796731742046395e1_f64) * t1087 * t12032 * t1043 * t1089 - F::cast_from(0.39512695097613069592e1_f64) * t1024 * t12073 * t3075 - F::cast_from(0.65854491829355115987e0_f64) * t43401 * t43348 * t43402 - F::cast_from(0.26341796731742046395e1_f64) * t1024 * t3291 * t11173;
    t43409
}
