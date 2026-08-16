//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1548/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1548<F: Float>(t3143: F, t42859: F, t342: F, t3154: F, t43351: F, t1089: F, t11788: F, t12066: F, t12073: F, t12128: F, t12150: F, t12163: F, t12167: F, t12168: F, t3059: F, t3204: F, t3287: F, t3288: F, t3304: F, t3318: F, t42610: F, t43292: F, t43348: F, t43438: F, t43439: F, t43443: F, t43446: F, t43450: F, t43453: F, t43456: F, t43467: F, t4976: F, t989: F) -> F {
    let t43471 = t42859 * t3143;
    let t43472 = t342 * t43471;
    let t43473 = t43351 * t3154;
    let t43480 = F::cast_from(0.15805078039045227836e2_f64) * t43438 * t43439 * t3304 + F::cast_from(0.15805078039045227836e2_f64) * t43443 * t12150 - F::cast_from(0.15805078039045227836e2_f64) * t43446 * t43292 * t1089 - F::cast_from(0.79025390195226139183e1_f64) * t43450 * t3288 + F::cast_from(0.79025390195226139183e1_f64) * t43453 * t12128 - F::cast_from(0.79025390195226139183e1_f64) * t43456 * t43439 * t3318 + F::cast_from(0.79025390195226139183e1_f64) * t3204 * t12073 * t3059 + F::cast_from(0.26341796731742046395e1_f64) * t989 * t12066 + F::cast_from(0.15805078039045227836e2_f64) * t11788 * t12163 + F::cast_from(0.15805078039045227836e2_f64) * t12167 * t43467 * t12168 + F::cast_from(0.92196288561097162379e1_f64) * t43472 * t43348 * t43473 - F::cast_from(0.26341796731742046395e1_f64) * t3287 * t42610 * t4976;
    t43480
}
