//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2924/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2924<F: Float>(t11120: F, t1695: F, t1071: F, t4743: F, t1078: F, t4772: F, t16237: F, t994: F, t11200: F, t1678: F, t1000: F, t1076: F, t1079: F, t1097: F, t11121: F, t11128: F, t11173: F, t11178: F, t11183: F, t11190: F, t11203: F, t11206: F, t12039: F, t12177: F, t12178: F, t16243: F, t16284: F, t16312: F, t16313: F, t16333: F, t16597: F, t16603: F, t16604: F, t3063: F, t3066: F, t3067: F, t3270: F, t3326: F, t43642: F, t4752: F, t4758: F, t4764: F, t4941: F, t5015: F, t995: F) -> (F, F) {
    let t53108 = t11120 * t1695;
    let t53119 = t4743 * t1071;
    let t53130 = t1078 * t4772;
    let t53157 = t994 * t16237;
    let t53160 = t11200 * t1678;
    let t53163 = F::cast_from(0.11853808529283920877e2_f64) * t16603 * t53108 * t12039 + F::cast_from(0.79025390195226139182e1_f64) * t16312 * t16604 * t12177 - F::cast_from(0.11853808529283920877e2_f64) * t1076 * t11121 * t5015 * t3270 - F::cast_from(0.39512695097613069591e1_f64) * t53119 * t1097 - F::cast_from(0.19756347548806534796e1_f64) * t16333 * t3326 - F::cast_from(0.39512695097613069591e1_f64) * t16284 * t12178 + F::cast_from(0.39512695097613069591e1_f64) * t4752 * t11178 + F::cast_from(0.39512695097613069591e1_f64) * t11128 * t4764 - F::cast_from(0.79025390195226139182e1_f64) * t16312 * t53130 * t3066 - F::cast_from(0.39512695097613069591e1_f64) * t16603 * t16604 * t11206 + F::cast_from(0.39512695097613069591e1_f64) * t3063 * t16243 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t1079 * t1695 * t11173 - F::cast_from(0.39512695097613069591e1_f64) * t16603 * t16604 * t11183 + F::cast_from(0.39512695097613069591e1_f64) * t16597 * t3067 + F::cast_from(0.39512695097613069591e1_f64) * t11128 * t4941 + F::cast_from(0.19756347548806534796e1_f64) * t11190 * t4764 + F::cast_from(0.39512695097613069591e1_f64) * t43642 * t4758 - F::cast_from(0.39512695097613069591e1_f64) * t16312 * t16313 * t11206 - F::cast_from(0.19756347548806534796e1_f64) * t53157 * t1000 - F::cast_from(0.39512695097613069591e1_f64) * t53160 * t11203;
    (t53108, t53163)
}
