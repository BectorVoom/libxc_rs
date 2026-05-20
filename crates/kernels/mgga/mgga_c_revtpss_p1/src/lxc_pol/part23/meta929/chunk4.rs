//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3037/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3037<F: Float>(t1647: F, t6343: F, t1678: F, t6235: F, t1079: F, t1096: F, t1097: F, t11224: F, t16302: F, t16305: F, t16312: F, t16313: F, t1652: F, t16600: F, t16603: F, t19400: F, t19414: F, t19428: F, t20152: F, t20175: F, t20194: F, t20214: F, t23598: F, t24044: F, t24048: F, t24061: F, t3264: F, t33754: F, t4758: F, t4935: F, t4947: F, t5016: F, t53174: F, t55464: F, t6245: F, t6251: F, t6259: F, t64737: F, t68022: F, t80028: F, t989: F, t995: F, t996: F) -> F {
    let t80983 = t1647 * t6343;
    let t80992 = t6235 * t1678;
    let t81015 = -F::cast_from(0.79025390195226139182e1_f64) * t16312 * t16313 * t20214 - F::cast_from(0.11853808529283920877e2_f64) * t53174 * t33754 * t19414 - F::cast_from(0.19756347548806534796e1_f64) * t4935 * t20152 - F::cast_from(0.19756347548806534796e1_f64) * t16305 * t6259 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t1079 * t23598 * t1096 - F::cast_from(0.19756347548806534796e1_f64) * t80983 * t1097 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t996 * t80028 - F::cast_from(0.79025390195226139182e1_f64) * t16603 * t19428 * t20194 - F::cast_from(0.19756347548806534796e1_f64) * t80992 * t1097 + F::cast_from(0.39512695097613069591e1_f64) * t16302 * t6251 + F::cast_from(0.79025390195226139182e1_f64) * t20175 * t4947 + F::cast_from(0.79025390195226139182e1_f64) * t16600 * t19400 + F::cast_from(0.39512695097613069591e1_f64) * t11224 * t24061 + F::cast_from(0.39512695097613069592e1_f64) * t68022 * t4758 + F::cast_from(0.39512695097613069591e1_f64) * t55464 * t6245 - F::cast_from(0.19756347548806534796e1_f64) * t64737 * t1652 - F::cast_from(0.39512695097613069591e1_f64) * t20175 * t5016 + F::cast_from(0.65854491829355115987e0_f64) * t989 * t24044 - F::cast_from(0.39512695097613069591e1_f64) * t3264 * t24048;
    t81015
}
