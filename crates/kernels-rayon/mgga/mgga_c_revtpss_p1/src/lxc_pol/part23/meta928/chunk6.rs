//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3027/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3027(t1082: f64, t1089: f64, t15670: f64, t15780: f64, t16381: f64, t16502: f64, t16544: f64, t1692: f64, t19457: f64, t19498: f64, t19509: f64, t19612: f64, t19856: f64, t24089: f64, t24104: f64, t3204: f64, t3278: f64, t3287: f64, t4977: f64, t4981: f64, t53877: f64, t6383: f64, t67927: f64, t78831: f64, t79480: f64, t79500: f64) -> f64 {
    let t80622 = 0.39512695097613069591e1_f64 * t4981 * t15780 * t24089 - 0.65854491829355115987e0_f64 * t3287 * t78831 * t1089 + 0.13170898365871023197e1_f64 * t3204 * t1082 * t79480 + 0.19756347548806534796e1_f64 * t3278 * t24104 - 0.19756347548806534796e1_f64 * t3287 * t79500 * t1089 - 0.19756347548806534796e1_f64 * t16502 * t19498 - 0.19756347548806534796e1_f64 * t16544 * t19612 - 0.39512695097613069591e1_f64 * t67927 * t4977 + 0.19756347548806534796e1_f64 * t19856 * t1692 + 0.79025390195226139182e1_f64 * t15670 * t19509 + 0.19756347548806534796e1_f64 * t16381 * t6383 - 0.11853808529283920877e2_f64 * t53877 * t19457;
    t80622
}
