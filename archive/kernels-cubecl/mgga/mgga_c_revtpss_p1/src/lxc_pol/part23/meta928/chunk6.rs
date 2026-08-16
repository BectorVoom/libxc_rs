//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3027/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3027<F: Float>(t1082: F, t1089: F, t15670: F, t15780: F, t16381: F, t16502: F, t16544: F, t1692: F, t19457: F, t19498: F, t19509: F, t19612: F, t19856: F, t24089: F, t24104: F, t3204: F, t3278: F, t3287: F, t4977: F, t4981: F, t53877: F, t6383: F, t67927: F, t78831: F, t79480: F, t79500: F) -> F {
    let t80622 = F::cast_from(0.39512695097613069591e1_f64) * t4981 * t15780 * t24089 - F::cast_from(0.65854491829355115987e0_f64) * t3287 * t78831 * t1089 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t1082 * t79480 + F::cast_from(0.19756347548806534796e1_f64) * t3278 * t24104 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t79500 * t1089 - F::cast_from(0.19756347548806534796e1_f64) * t16502 * t19498 - F::cast_from(0.19756347548806534796e1_f64) * t16544 * t19612 - F::cast_from(0.39512695097613069591e1_f64) * t67927 * t4977 + F::cast_from(0.19756347548806534796e1_f64) * t19856 * t1692 + F::cast_from(0.79025390195226139182e1_f64) * t15670 * t19509 + F::cast_from(0.19756347548806534796e1_f64) * t16381 * t6383 - F::cast_from(0.11853808529283920877e2_f64) * t53877 * t19457;
    t80622
}
