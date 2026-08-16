//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1554/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1554<F: Float>(t12032: F, t994: F, t3259: F, t989: F, t1071: F, t11213: F, t378: F, t42277: F, t1000: F, t1076: F, t1096: F, t1097: F, t11121: F, t11123: F, t11174: F, t11187: F, t11195: F, t11207: F, t11210: F, t11214: F, t12034: F, t12043: F, t12173: F, t3047: F, t3058: F, t3059: F, t3063: F, t3067: F, t3075: F, t3264: F, t3269: F, t3270: F, t3271: F, t3325: F, t3326: F, t995: F) -> F {
    let t43670 = t994 * t12032;
    let t43687 = t989 * t3259;
    let t43696 = t11213 * t1071;
    let t43707 = t42277 * t378;
    let t43714 = -F::cast_from(0.26341796731742046395e1_f64) * t3047 * t11174 - F::cast_from(0.26341796731742046395e1_f64) * t43670 * t1000 - F::cast_from(0.39512695097613069592e1_f64) * t11210 * t3326 + F::cast_from(0.52683593463484092788e1_f64) * t1076 * t3269 * t1096 * t12173 + F::cast_from(0.79025390195226139183e1_f64) * t11214 * t3067 + F::cast_from(0.15805078039045227836e2_f64) * t3058 * t3269 * t3059 * t3270 - F::cast_from(0.15805078039045227836e2_f64) * t3264 * t11123 - F::cast_from(0.79025390195226139183e1_f64) * t43687 * t1097 - F::cast_from(0.23707617058567841754e2_f64) * t1076 * t11121 * t3270 * t3325 + F::cast_from(0.79025390195226139183e1_f64) * t11195 * t3271 - F::cast_from(0.79025390195226139183e1_f64) * t43696 * t1000 - F::cast_from(0.79025390195226139183e1_f64) * t995 * t3269 * t3075 * t3270 - F::cast_from(0.39512695097613069592e1_f64) * t11195 * t3326 + F::cast_from(0.79025390195226139183e1_f64) * t3063 * t11207 - F::cast_from(0.26341796731742046395e1_f64) * t43707 * t1000 + F::cast_from(0.26341796731742046395e1_f64) * t989 * t12034 + F::cast_from(0.15805078039045227836e2_f64) * t11187 * t12043;
    t43714
}
