//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1552/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1552(t12153: f64, t3057: f64, t3043: f64, t3316: f64, t1071: f64, t1087: f64, t1089: f64, t1093: f64, t11620: f64, t11687: f64, t11902: f64, t12057: f64, t12078: f64, t12079: f64, t12094: f64, t12127: f64, t12143: f64, t12150: f64, t12154: f64, t15604: f64, t16506: f64, t16523: f64, t3278: f64, t3299: f64, t3302: f64, t3304: f64, t3319: f64, t3322: f64, t43334: f64, t43467: f64) -> f64 {
    let t43598 = t3057 * t12153;
    let t43611 = t3043 * t3316;
    let t43626 = 0.26341796731742046395e1_f64 * t1087 * t1071 * t11620 * t1089 + 0.15805078039045227836e2_f64 * t43598 * t12150 - 0.79025390195226139184e1_f64 * t16523 * t12094 - 0.79025390195226139184e1_f64 * t16506 * t12094 + 0.39512695097613069592e1_f64 * t3043 * t3322 + 0.26341796731742046395e1_f64 * t11902 * t1093 - 0.79025390195226139183e1_f64 * t12154 * t12143 - 0.39512695097613069592e1_f64 * t43611 * t3319 + 0.79025390195226139183e1_f64 * t3278 * t12057 + 0.79025390195226139184e1_f64 * t12127 * t11687 * t3302 * t15604 - 0.15805078039045227836e2_f64 * t12078 * t43467 * t12079 + 0.39512695097613069591e1_f64 * t3299 * t43334 * t3304;
    t43626
}
