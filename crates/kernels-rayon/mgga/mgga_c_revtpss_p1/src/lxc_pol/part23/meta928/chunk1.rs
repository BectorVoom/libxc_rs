//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3022/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3022(t1043: f64, t1071: f64, t1087: f64, t1089: f64, t12146: f64, t12149: f64, t12154: f64, t19479: f64, t19521: f64, t19566: f64, t19603: f64, t23820: f64, t23964: f64, t24093: f64, t24108: f64, t24135: f64, t3278: f64, t3287: f64, t43420: f64, t4954: f64, t4961: f64, t4976: f64, t4992: f64, t55985: f64, t6386: f64, t67501: f64, t78641: f64, t79159: f64) -> f64 {
    let t80458 = -0.19756347548806534796e1_f64 * t55985 * t6386 + 0.19756347548806534796e1_f64 * t19566 * t4992 + 0.79025390195226139182e1_f64 * t19603 * t19521 + 0.65854491829355115987e0_f64 * t3278 * t24108 + 0.39512695097613069591e1_f64 * t12149 * t23964 * t1043 * t1089 + 0.19756347548806534796e1_f64 * t4954 * t19479 - 0.65854491829355115987e0_f64 * t3287 * t79159 * t4976 - 0.19756347548806534796e1_f64 * t12146 * t24135 - 0.19756347548806534796e1_f64 * t12154 * t24135 - 0.19756347548806534796e1_f64 * t3287 * t78641 * t1089 + 0.39512695097613069592e1_f64 * t67501 * t4961 + 0.65854491829355115987e0_f64 * t1087 * t1071 * t23820 * t1089 + 0.39512695097613069591e1_f64 * t43420 * t24093;
    t80458
}
