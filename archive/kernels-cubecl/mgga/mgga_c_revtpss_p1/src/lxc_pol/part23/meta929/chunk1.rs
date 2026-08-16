//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3034/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3034<F: Float>(t23959: F, t378: F, t1076: F, t1079: F, t1096: F, t1097: F, t11201: F, t16284: F, t16333: F, t1652: F, t16600: F, t1696: F, t19351: F, t19415: F, t19421: F, t20152: F, t20172: F, t20215: F, t20219: F, t23617: F, t24177: F, t3047: F, t3058: F, t3063: F, t3269: F, t4747: F, t4752: F, t4935: F, t4940: F, t4947: F, t5015: F, t53174: F, t53281: F, t6244: F, t6245: F, t6258: F, t6393: F, t64555: F, t64817: F, t68138: F, t78740: F, t995: F, t996: F) -> F {
    let t80833 = t23959 * t378;
    let t80869 = -F::cast_from(0.11853808529283920877e2_f64) * t11201 * t996 * t78740 - F::cast_from(0.39512695097613069591e1_f64) * t16600 * t19421 - F::cast_from(0.19756347548806534796e1_f64) * t68138 * t1696 + F::cast_from(0.39512695097613069591e1_f64) * t53281 * t6245 - F::cast_from(0.19756347548806534796e1_f64) * t4752 * t20152 + F::cast_from(0.39512695097613069592e1_f64) * t16600 * t19415 - F::cast_from(0.65854491829355115987e0_f64) * t80833 * t1097 + F::cast_from(0.39512695097613069592e1_f64) * t16284 * t19415 + F::cast_from(0.11853808529283920877e2_f64) * t53174 * t64555 * t4940 - F::cast_from(0.39512695097613069591e1_f64) * t3058 * t1079 * t6244 * t5015 + F::cast_from(0.19756347548806534796e1_f64) * t4747 * t20219 + F::cast_from(0.19756347548806534796e1_f64) * t3047 * t23617 - F::cast_from(0.19756347548806534796e1_f64) * t16333 * t6393 - F::cast_from(0.19756347548806534796e1_f64) * t64817 * t1652 + F::cast_from(0.19756347548806534796e1_f64) * t3063 * t23617 + F::cast_from(0.39512695097613069591e1_f64) * t4747 * t20215 + F::cast_from(0.39512695097613069592e1_f64) * t19351 * t4947 + F::cast_from(0.13170898365871023197e1_f64) * t1076 * t3269 * t24177 * t1096 + F::cast_from(0.39512695097613069592e1_f64) * t4935 * t20172 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t6258 * t5015;
    t80869
}
