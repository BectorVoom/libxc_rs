//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1554/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1554(t12032: f64, t994: f64, t3259: f64, t989: f64, t1071: f64, t11213: f64, t378: f64, t42277: f64, t1000: f64, t1076: f64, t1096: f64, t1097: f64, t11121: f64, t11123: f64, t11174: f64, t11187: f64, t11195: f64, t11207: f64, t11210: f64, t11214: f64, t12034: f64, t12043: f64, t12173: f64, t3047: f64, t3058: f64, t3059: f64, t3063: f64, t3067: f64, t3075: f64, t3264: f64, t3269: f64, t3270: f64, t3271: f64, t3325: f64, t3326: f64, t995: f64) -> f64 {
    let t43670 = t994 * t12032;
    let t43687 = t989 * t3259;
    let t43696 = t11213 * t1071;
    let t43707 = t42277 * t378;
    let t43714 = -0.26341796731742046395e1_f64 * t3047 * t11174 - 0.26341796731742046395e1_f64 * t43670 * t1000 - 0.39512695097613069592e1_f64 * t11210 * t3326 + 0.52683593463484092788e1_f64 * t1076 * t3269 * t1096 * t12173 + 0.79025390195226139183e1_f64 * t11214 * t3067 + 0.15805078039045227836e2_f64 * t3058 * t3269 * t3059 * t3270 - 0.15805078039045227836e2_f64 * t3264 * t11123 - 0.79025390195226139183e1_f64 * t43687 * t1097 - 0.23707617058567841754e2_f64 * t1076 * t11121 * t3270 * t3325 + 0.79025390195226139183e1_f64 * t11195 * t3271 - 0.79025390195226139183e1_f64 * t43696 * t1000 - 0.79025390195226139183e1_f64 * t995 * t3269 * t3075 * t3270 - 0.39512695097613069592e1_f64 * t11195 * t3326 + 0.79025390195226139183e1_f64 * t3063 * t11207 - 0.26341796731742046395e1_f64 * t43707 * t1000 + 0.26341796731742046395e1_f64 * t989 * t12034 + 0.15805078039045227836e2_f64 * t11187 * t12043;
    t43714
}
