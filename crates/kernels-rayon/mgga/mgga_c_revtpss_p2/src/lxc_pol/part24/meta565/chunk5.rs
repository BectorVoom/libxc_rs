//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1719/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1719(t1024: f64, t11940: f64, t12052: f64, t15670: f64, t1647: f64, t1651: f64, t16566: f64, t19450: f64, t23964: f64, t24031: f64, t24075: f64, t24162: f64, t3204: f64, t3304: f64, t3318: f64, t43341: f64, t43401: f64, t43402: f64, t43438: f64, t43456: f64, t43472: f64, t43473: f64, t5004: f64, t6299: f64, t80350: f64, t80396: f64, t88794: f64, t89084: f64, t89647: f64) -> f64 {
    let t89697 = -0.65854491829355115987e0_f64 * t43401 * t89647 * t43402 + 0.92196288561097162379e1_f64 * t43472 * t89647 * t43473 - 0.15805078039045227836e2_f64 * t11940 * t5004 * t24031 + 0.15805078039045227836e2_f64 * t3204 * t5004 * t23964 - 0.26341796731742046395e1_f64 * t1024 * t80396 * t1651 + 0.26341796731742046395e1_f64 * t1647 * t24162 - 0.26341796731742046395e1_f64 * t43341 * t88794 * t12052 + 0.15805078039045227836e2_f64 * t15670 * t24075 + 0.39512695097613069592e1_f64 * t16566 * t19450 * t80350 * t6299 + 0.15805078039045227836e2_f64 * t43438 * t89084 * t3304 - 0.79025390195226139183e1_f64 * t43456 * t89084 * t3318;
    t89697
}
