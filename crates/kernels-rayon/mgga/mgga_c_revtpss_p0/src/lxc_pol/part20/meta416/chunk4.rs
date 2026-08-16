//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1551/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1551(t11620: f64, t4982: f64, t16553: f64, t3133: f64, t12077: f64, t989: f64, t1082: f64, t1087: f64, t1089: f64, t11804: f64, t12047: f64, t12052: f64, t12074: f64, t12080: f64, t12131: f64, t12143: f64, t12146: f64, t12154: f64, t12157: f64, t16552: f64, t3204: f64, t3223: f64, t3259: f64, t3291: f64, t3317: f64, t3318: f64, t42047: f64, t42804: f64, t43467: f64, t43497: f64, t4981: f64) -> f64 {
    let t43562 = t4982 * t11620;
    let t43568 = t16553 * t3133;
    let t43574 = t989 * t12077;
    let t43593 = -0.79025390195226139183e1_f64 * t3223 * t12074 + 0.52683593463484092788e1_f64 * t4981 * t12131 * t43562 - 0.79025390195226139183e1_f64 * t12154 * t12157 + 0.23707617058567841754e2_f64 * t16552 * t42804 * t43568 - 0.79025390195226139183e1_f64 * t12146 * t12143 - 0.15805078039045227836e2_f64 * t43574 * t12080 + 0.26341796731742046395e1_f64 * t12047 * t43467 * t12052 + 0.15805078039045227836e2_f64 * t3204 * t3291 * t11804 - 0.39512695097613069592e1_f64 * t3317 * t43497 * t3318 + 0.39512695097613069592e1_f64 * t1087 * t3259 * t3133 * t1089 + 0.39512695097613069591e1_f64 * t3204 * t1082 * t42047;
    t43593
}
