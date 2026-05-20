//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3022/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3022<F: Float>(t1043: F, t1071: F, t1087: F, t1089: F, t12146: F, t12149: F, t12154: F, t19479: F, t19521: F, t19566: F, t19603: F, t23820: F, t23964: F, t24093: F, t24108: F, t24135: F, t3278: F, t3287: F, t43420: F, t4954: F, t4961: F, t4976: F, t4992: F, t55985: F, t6386: F, t67501: F, t78641: F, t79159: F) -> F {
    let t80458 = -F::cast_from(0.19756347548806534796e1_f64) * t55985 * t6386 + F::cast_from(0.19756347548806534796e1_f64) * t19566 * t4992 + F::cast_from(0.79025390195226139182e1_f64) * t19603 * t19521 + F::cast_from(0.65854491829355115987e0_f64) * t3278 * t24108 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t23964 * t1043 * t1089 + F::cast_from(0.19756347548806534796e1_f64) * t4954 * t19479 - F::cast_from(0.65854491829355115987e0_f64) * t3287 * t79159 * t4976 - F::cast_from(0.19756347548806534796e1_f64) * t12146 * t24135 - F::cast_from(0.19756347548806534796e1_f64) * t12154 * t24135 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t78641 * t1089 + F::cast_from(0.39512695097613069592e1_f64) * t67501 * t4961 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t1071 * t23820 * t1089 + F::cast_from(0.39512695097613069591e1_f64) * t43420 * t24093;
    t80458
}
