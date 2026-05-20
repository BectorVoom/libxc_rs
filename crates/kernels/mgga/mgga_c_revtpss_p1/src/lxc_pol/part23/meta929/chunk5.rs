//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3038/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3038<F: Float>(t24042: F, t342: F, t1076: F, t1079: F, t1096: F, t1097: F, t11201: F, t16284: F, t16302: F, t16374: F, t16603: F, t1695: F, t1696: F, t19381: F, t19396: F, t19421: F, t19429: F, t20151: F, t20171: F, t20195: F, t20204: F, t20219: F, t23599: F, t24031: F, t3058: F, t3063: F, t3268: F, t3269: F, t4747: F, t4764: F, t4772: F, t4778: F, t4932: F, t4935: F, t4946: F, t53108: F, t56087: F, t6235: F, t6258: F, t6259: F, t6392: F, t64547: F, t68185: F, t78826: F, t79084: F, t995: F, t996: F) -> F {
    let t81052 = t342 * t24042;
    let t81068 = -F::cast_from(0.39512695097613069592e1_f64) * t16603 * t3268 * t6258 * t4946 - F::cast_from(0.19756347548806534796e1_f64) * t68185 * t1696 - F::cast_from(0.19756347548806534796e1_f64) * t16302 * t6259 + F::cast_from(0.19756347548806534796e1_f64) * t6235 * t4932 - F::cast_from(0.79025390195226139182e1_f64) * t64547 * t19429 + F::cast_from(0.39512695097613069591e1_f64) * t4747 * t19396 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t4772 * t6392 + F::cast_from(0.79025390195226139182e1_f64) * t4935 * t20195 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t996 * t79084 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t996 * t78826 - F::cast_from(0.19756347548806534796e1_f64) * t16374 * t6259 + F::cast_from(0.19756347548806534796e1_f64) * t20204 * t4764 - F::cast_from(0.65854491829355115987e0_f64) * t3063 * t23599 - F::cast_from(0.19756347548806534796e1_f64) * t4747 * t19381 - F::cast_from(0.39512695097613069591e1_f64) * t16284 * t19421 - F::cast_from(0.65854491829355115987e0_f64) * t81052 * t1097 + F::cast_from(0.39512695097613069591e1_f64) * t11201 * t1079 * t24031 * t1096 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t3269 * t1695 * t20151 - F::cast_from(0.11853808529283920877e2_f64) * t56087 * t53108 * t20171 + F::cast_from(0.19756347548806534796e1_f64) * t4778 * t20219;
    t81068
}
