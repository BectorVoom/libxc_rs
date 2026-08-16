//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1719/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1719<F: Float>(t1024: F, t11940: F, t12052: F, t15670: F, t1647: F, t1651: F, t16566: F, t19450: F, t23964: F, t24031: F, t24075: F, t24162: F, t3204: F, t3304: F, t3318: F, t43341: F, t43401: F, t43402: F, t43438: F, t43456: F, t43472: F, t43473: F, t5004: F, t6299: F, t80350: F, t80396: F, t88794: F, t89084: F, t89647: F) -> F {
    let t89697 = -F::cast_from(0.65854491829355115987e0_f64) * t43401 * t89647 * t43402 + F::cast_from(0.92196288561097162379e1_f64) * t43472 * t89647 * t43473 - F::cast_from(0.15805078039045227836e2_f64) * t11940 * t5004 * t24031 + F::cast_from(0.15805078039045227836e2_f64) * t3204 * t5004 * t23964 - F::cast_from(0.26341796731742046395e1_f64) * t1024 * t80396 * t1651 + F::cast_from(0.26341796731742046395e1_f64) * t1647 * t24162 - F::cast_from(0.26341796731742046395e1_f64) * t43341 * t88794 * t12052 + F::cast_from(0.15805078039045227836e2_f64) * t15670 * t24075 + F::cast_from(0.39512695097613069592e1_f64) * t16566 * t19450 * t80350 * t6299 + F::cast_from(0.15805078039045227836e2_f64) * t43438 * t89084 * t3304 - F::cast_from(0.79025390195226139183e1_f64) * t43456 * t89084 * t3318;
    t89697
}
