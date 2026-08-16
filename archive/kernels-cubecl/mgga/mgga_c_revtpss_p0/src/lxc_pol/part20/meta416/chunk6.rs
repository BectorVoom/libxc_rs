//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1553/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1553<F: Float>(t1071: F, t11200: F, t378: F, t42358: F, t11223: F, t1076: F, t1079: F, t1096: F, t11173: F, t11190: F, t11201: F, t11202: F, t11203: F, t11207: F, t11210: F, t11214: F, t11220: F, t11224: F, t12043: F, t12173: F, t12174: F, t225: F, t3047: F, t3058: F, t3060: F, t3067: F, t3076: F, t3264: F, t3271: F, t3326: F, t342: F, t385: F, t42909: F, t43323: F, t43374: F, t43409: F, t43437: F, t43480: F, t43519: F, t43558: F, t43593: F, t43626: F, t995: F, t996: F, t999: F) -> F {
    let t43637 = t11200 * t1071;
    let t43642 = t42358 * t378;
    let t43656 = t11223 * t1071;
    let t43667 = F::cast_from(0.26341796731742046395e1_f64) * t995 * t1079 * t11173 * t1096 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t43323 * t225 * t385 - F::cast_from(0.39512695097613069592e1_f64) * t11214 * t3076 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1079 * (t43374 + t43409 + t43437 + t43480 + t43519 + t43558 + t43593 + t43626) - F::cast_from(0.79025390195226139183e1_f64) * t11220 * t3326 + F::cast_from(0.15805078039045227836e2_f64) * t11224 * t12043 - F::cast_from(0.15805078039045227836e2_f64) * t43637 * t11203 + F::cast_from(0.15805078039045227836e2_f64) * t11220 * t3271 + F::cast_from(0.79025390195226139183e1_f64) * t43642 * t3060 + F::cast_from(0.26341796731742046395e1_f64) * t995 * t1079 * t999 * t12173 + F::cast_from(0.52683593463484092788e1_f64) * t3058 * t996 * t42909 + F::cast_from(0.15805078039045227836e2_f64) * t11201 * t1079 * t11202 * t1096 + F::cast_from(0.15805078039045227836e2_f64) * t43656 * t3060 + F::cast_from(0.79025390195226139183e1_f64) * t11190 * t3067 + F::cast_from(0.79025390195226139183e1_f64) * t11210 * t3271 - F::cast_from(0.26341796731742046395e1_f64) * t3264 * t12174 + F::cast_from(0.79025390195226139183e1_f64) * t3047 * t11207;
    t43667
}
