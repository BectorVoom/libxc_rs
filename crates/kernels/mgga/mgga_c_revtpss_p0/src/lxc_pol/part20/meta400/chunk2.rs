//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1485/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1485<F: Float>(t42083: F, t42096: F, t3057: F, t3259: F, t1000: F, t1073: F, t1076: F, t1097: F, t11121: F, t11122: F, t11128: F, t11201: F, t11203: F, t11224: F, t11902: F, t12040: F, t12174: F, t12178: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t386: F, t42001: F, t42033: F, t42038: F, t42041: F, t42044: F, t42047: F, t42052: F, t42060: F, t42061: F, t42067: F, t42068: F, t995: F, t996: F, t999: F) -> (F, F) {
    let t42097 = t42083 + t42096;
    let t42107 = t3057 * t3259;
    let t42112 = -F::cast_from(0.23707617058567841754e2_f64) * t11201 * t996 * t42001 + F::cast_from(0.15805078039045227836e2_f64) * t11128 * t3067 + F::cast_from(0.65854491829355115987e0_f64) * t42033 * t386 + F::cast_from(0.26341796731742046395e1_f64) * t11902 * t1073 - F::cast_from(0.26341796731742046395e1_f64) * t42038 * t1097 - F::cast_from(0.26341796731742046395e1_f64) * t42041 * t1097 - F::cast_from(0.79025390195226139183e1_f64) * t42044 * t1000 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t996 * t42047 - F::cast_from(0.15805078039045227836e2_f64) * t42052 * t11203 - F::cast_from(0.15805078039045227836e2_f64) * t11224 * t12178 + F::cast_from(0.15805078039045227836e2_f64) * t42060 * t996 * t42061 + F::cast_from(0.15805078039045227836e2_f64) * t1076 * t42067 * t42068 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t996 * t42097 + F::cast_from(0.15805078039045227836e2_f64) * t995 * t11121 * t999 * t11122 - F::cast_from(0.15805078039045227836e2_f64) * t3063 * t12040 + F::cast_from(0.79025390195226139183e1_f64) * t42107 * t3060 - F::cast_from(0.26341796731742046395e1_f64) * t3052 * t12174;
    (t42097, t42112)
}
