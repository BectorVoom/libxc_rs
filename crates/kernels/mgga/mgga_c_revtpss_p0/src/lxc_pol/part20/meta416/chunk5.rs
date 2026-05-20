//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1552/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1552<F: Float>(t12153: F, t3057: F, t3043: F, t3316: F, t1071: F, t1087: F, t1089: F, t1093: F, t11620: F, t11687: F, t11902: F, t12057: F, t12078: F, t12079: F, t12094: F, t12127: F, t12143: F, t12150: F, t12154: F, t15604: F, t16506: F, t16523: F, t3278: F, t3299: F, t3302: F, t3304: F, t3319: F, t3322: F, t43334: F, t43467: F) -> F {
    let t43598 = t3057 * t12153;
    let t43611 = t3043 * t3316;
    let t43626 = F::cast_from(0.26341796731742046395e1_f64) * t1087 * t1071 * t11620 * t1089 + F::cast_from(0.15805078039045227836e2_f64) * t43598 * t12150 - F::cast_from(0.79025390195226139184e1_f64) * t16523 * t12094 - F::cast_from(0.79025390195226139184e1_f64) * t16506 * t12094 + F::cast_from(0.39512695097613069592e1_f64) * t3043 * t3322 + F::cast_from(0.26341796731742046395e1_f64) * t11902 * t1093 - F::cast_from(0.79025390195226139183e1_f64) * t12154 * t12143 - F::cast_from(0.39512695097613069592e1_f64) * t43611 * t3319 + F::cast_from(0.79025390195226139183e1_f64) * t3278 * t12057 + F::cast_from(0.79025390195226139184e1_f64) * t12127 * t11687 * t3302 * t15604 - F::cast_from(0.15805078039045227836e2_f64) * t12078 * t43467 * t12079 + F::cast_from(0.39512695097613069591e1_f64) * t3299 * t43334 * t3304;
    t43626
}
