//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1546/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1546<F: Float>(t1086: F, t11902: F, t12166: F, t989: F, t16409: F, t994: F, t1082: F, t1090: F, t11782: F, t12070: F, t12097: F, t12105: F, t12124: F, t12137: F, t12146: F, t12157: F, t12169: F, t3278: F, t3295: F, t3313: F, t381: F, t42033: F, t42061: F, t42261: F, t43154: F) -> F {
    let t43413 = t11902 * t1086;
    let t43420 = t989 * t12166;
    let t43432 = t994 * t16409;
    let t43437 = -F::cast_from(0.39512695097613069592e1_f64) * t11782 * t3295 + F::cast_from(0.26341796731742046395e1_f64) * t43413 * t1090 + F::cast_from(0.65854491829355115987e0_f64) * t42033 * t381 - F::cast_from(0.15805078039045227836e2_f64) * t42261 * t12105 + F::cast_from(0.15805078039045227836e2_f64) * t43420 * t12169 + F::cast_from(0.39512695097613069592e1_f64) * t12097 * t3313 + F::cast_from(0.15805078039045227836e2_f64) * t43154 * t1082 * t42061 + F::cast_from(0.79025390195226139183e1_f64) * t3278 * t12137 + F::cast_from(0.26341796731742046395e1_f64) * t3278 * t12070 - F::cast_from(0.15805078039045227836e2_f64) * t43432 * t12124 - F::cast_from(0.79025390195226139183e1_f64) * t12146 * t12157;
    t43437
}
