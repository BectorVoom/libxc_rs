//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1548/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1548(t3143: f64, t42859: f64, t342: f64, t3154: f64, t43351: f64, t1089: f64, t11788: f64, t12066: f64, t12073: f64, t12128: f64, t12150: f64, t12163: f64, t12167: f64, t12168: f64, t3059: f64, t3204: f64, t3287: f64, t3288: f64, t3304: f64, t3318: f64, t42610: f64, t43292: f64, t43348: f64, t43438: f64, t43439: f64, t43443: f64, t43446: f64, t43450: f64, t43453: f64, t43456: f64, t43467: f64, t4976: f64, t989: f64) -> f64 {
    let t43471 = t42859 * t3143;
    let t43472 = t342 * t43471;
    let t43473 = t43351 * t3154;
    let t43480 = 0.15805078039045227836e2_f64 * t43438 * t43439 * t3304 + 0.15805078039045227836e2_f64 * t43443 * t12150 - 0.15805078039045227836e2_f64 * t43446 * t43292 * t1089 - 0.79025390195226139183e1_f64 * t43450 * t3288 + 0.79025390195226139183e1_f64 * t43453 * t12128 - 0.79025390195226139183e1_f64 * t43456 * t43439 * t3318 + 0.79025390195226139183e1_f64 * t3204 * t12073 * t3059 + 0.26341796731742046395e1_f64 * t989 * t12066 + 0.15805078039045227836e2_f64 * t11788 * t12163 + 0.15805078039045227836e2_f64 * t12167 * t43467 * t12168 + 0.92196288561097162379e1_f64 * t43472 * t43348 * t43473 - 0.26341796731742046395e1_f64 * t3287 * t42610 * t4976;
    t43480
}
