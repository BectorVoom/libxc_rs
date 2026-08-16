//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1546/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1546(t1086: f64, t11902: f64, t12166: f64, t989: f64, t16409: f64, t994: f64, t1082: f64, t1090: f64, t11782: f64, t12070: f64, t12097: f64, t12105: f64, t12124: f64, t12137: f64, t12146: f64, t12157: f64, t12169: f64, t3278: f64, t3295: f64, t3313: f64, t381: f64, t42033: f64, t42061: f64, t42261: f64, t43154: f64) -> f64 {
    let t43413 = t11902 * t1086;
    let t43420 = t989 * t12166;
    let t43432 = t994 * t16409;
    let t43437 = -0.39512695097613069592e1_f64 * t11782 * t3295 + 0.26341796731742046395e1_f64 * t43413 * t1090 + 0.65854491829355115987e0_f64 * t42033 * t381 - 0.15805078039045227836e2_f64 * t42261 * t12105 + 0.15805078039045227836e2_f64 * t43420 * t12169 + 0.39512695097613069592e1_f64 * t12097 * t3313 + 0.15805078039045227836e2_f64 * t43154 * t1082 * t42061 + 0.79025390195226139183e1_f64 * t3278 * t12137 + 0.26341796731742046395e1_f64 * t3278 * t12070 - 0.15805078039045227836e2_f64 * t43432 * t12124 - 0.79025390195226139183e1_f64 * t12146 * t12157;
    t43437
}
