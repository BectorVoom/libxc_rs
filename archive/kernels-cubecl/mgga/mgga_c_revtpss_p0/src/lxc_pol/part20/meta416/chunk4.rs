//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1551/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1551<F: Float>(t11620: F, t4982: F, t16553: F, t3133: F, t12077: F, t989: F, t1082: F, t1087: F, t1089: F, t11804: F, t12047: F, t12052: F, t12074: F, t12080: F, t12131: F, t12143: F, t12146: F, t12154: F, t12157: F, t16552: F, t3204: F, t3223: F, t3259: F, t3291: F, t3317: F, t3318: F, t42047: F, t42804: F, t43467: F, t43497: F, t4981: F) -> F {
    let t43562 = t4982 * t11620;
    let t43568 = t16553 * t3133;
    let t43574 = t989 * t12077;
    let t43593 = -F::cast_from(0.79025390195226139183e1_f64) * t3223 * t12074 + F::cast_from(0.52683593463484092788e1_f64) * t4981 * t12131 * t43562 - F::cast_from(0.79025390195226139183e1_f64) * t12154 * t12157 + F::cast_from(0.23707617058567841754e2_f64) * t16552 * t42804 * t43568 - F::cast_from(0.79025390195226139183e1_f64) * t12146 * t12143 - F::cast_from(0.15805078039045227836e2_f64) * t43574 * t12080 + F::cast_from(0.26341796731742046395e1_f64) * t12047 * t43467 * t12052 + F::cast_from(0.15805078039045227836e2_f64) * t3204 * t3291 * t11804 - F::cast_from(0.39512695097613069592e1_f64) * t3317 * t43497 * t3318 + F::cast_from(0.39512695097613069592e1_f64) * t1087 * t3259 * t3133 * t1089 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t1082 * t42047;
    t43593
}
