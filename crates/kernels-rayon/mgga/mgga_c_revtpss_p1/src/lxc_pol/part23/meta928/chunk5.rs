//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3026/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3026(t1043: f64, t1087: f64, t1089: f64, t12149: f64, t15655: f64, t16449: f64, t19477: f64, t19492: f64, t19549: f64, t19557: f64, t19594: f64, t19597: f64, t20133: f64, t23992: f64, t24042: f64, t3204: f64, t4857: f64, t4893: f64, t4976: f64, t4981: f64, t4982: f64, t4983: f64, t55701: f64, t55988: f64, t55991: f64, t6244: f64, t6365: f64, t6371: f64, t67969: f64, t67972: f64, t73: f64, t78873: f64) -> f64 {
    let t80592 = 0.65854491829355115987e0_f64 * t1087 * t24042 * t1043 * t1089 + 0.13170898365871023197e1_f64 * t4981 * t78873 * t4983 + 0.39512695097613069591e1_f64 * t4981 * t4893 * t4982 * t19477 + 0.11853808529283920877e2_f64 * t67969 * t19549 - 0.11853808529283920877e2_f64 * t67972 * t19492 - 0.39512695097613069591e1_f64 * t55701 * t6365 + 0.39512695097613069591e1_f64 * t3204 * t16449 * t6244 - 0.79025390195226139182e1_f64 * t55988 * t19594 + 0.39512695097613069591e1_f64 * t55991 * t19597 + 0.39512695097613069592e1_f64 * t12149 * t23992 * t73 * t4976 - 0.19756347548806534796e1_f64 * t15655 * t6371 - 0.19756347548806534796e1_f64 * t4857 * t19557 - 0.39512695097613069591e1_f64 * t4857 * t20133;
    t80592
}
