//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3026/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3026<F: Float>(t1043: F, t1087: F, t1089: F, t12149: F, t15655: F, t16449: F, t19477: F, t19492: F, t19549: F, t19557: F, t19594: F, t19597: F, t20133: F, t23992: F, t24042: F, t3204: F, t4857: F, t4893: F, t4976: F, t4981: F, t4982: F, t4983: F, t55701: F, t55988: F, t55991: F, t6244: F, t6365: F, t6371: F, t67969: F, t67972: F, t73: F, t78873: F) -> F {
    let t80592 = F::cast_from(0.65854491829355115987e0_f64) * t1087 * t24042 * t1043 * t1089 + F::cast_from(0.13170898365871023197e1_f64) * t4981 * t78873 * t4983 + F::cast_from(0.39512695097613069591e1_f64) * t4981 * t4893 * t4982 * t19477 + F::cast_from(0.11853808529283920877e2_f64) * t67969 * t19549 - F::cast_from(0.11853808529283920877e2_f64) * t67972 * t19492 - F::cast_from(0.39512695097613069591e1_f64) * t55701 * t6365 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t16449 * t6244 - F::cast_from(0.79025390195226139182e1_f64) * t55988 * t19594 + F::cast_from(0.39512695097613069591e1_f64) * t55991 * t19597 + F::cast_from(0.39512695097613069592e1_f64) * t12149 * t23992 * t73 * t4976 - F::cast_from(0.19756347548806534796e1_f64) * t15655 * t6371 - F::cast_from(0.19756347548806534796e1_f64) * t4857 * t19557 - F::cast_from(0.39512695097613069591e1_f64) * t4857 * t20133;
    t80592
}
