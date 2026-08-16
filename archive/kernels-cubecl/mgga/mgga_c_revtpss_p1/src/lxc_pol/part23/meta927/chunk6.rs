//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3015/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3015<F: Float>(t11187: F, t11224: F, t16333: F, t16362: F, t1647: F, t16603: F, t16604: F, t1696: F, t19342: F, t19384: F, t19385: F, t19428: F, t20168: F, t20171: F, t20211: F, t20215: F, t225: F, t23583: F, t23607: F, t24048: F, t24061: F, t3047: F, t3052: F, t3269: F, t342: F, t385: F, t4743: F, t4747: F, t4764: F, t4772: F, t4773: F, t4778: F, t6345: F, t6350: F, t6351: F, t6393: F, t64605: F, t64639: F, t80132: F, t995: F) -> F {
    let t80166 = -F::cast_from(0.39512695097613069592e1_f64) * t16603 * t19428 * t20171 - F::cast_from(0.39512695097613069591e1_f64) * t64639 * t1696 + F::cast_from(0.19756347548806534796e1_f64) * t20211 * t4764 + F::cast_from(0.39512695097613069591e1_f64) * t4778 * t20215 - F::cast_from(0.39512695097613069591e1_f64) * t11224 * t23583 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t80132 * t225 * t385 - F::cast_from(0.19756347548806534796e1_f64) * t20211 * t4773 + F::cast_from(0.39512695097613069591e1_f64) * t11187 * t24061 + F::cast_from(0.19756347548806534796e1_f64) * t4743 * t6345 - F::cast_from(0.39512695097613069591e1_f64) * t16603 * t16604 * t19384 - F::cast_from(0.39512695097613069591e1_f64) * t995 * t3269 * t4772 * t6350 - F::cast_from(0.39512695097613069591e1_f64) * t4747 * t19342 - F::cast_from(0.39512695097613069591e1_f64) * t64605 * t1696 + F::cast_from(0.19756347548806534796e1_f64) * t4778 * t19385 + F::cast_from(0.39512695097613069591e1_f64) * t16333 * t6351 - F::cast_from(0.39512695097613069591e1_f64) * t3052 * t24048 + F::cast_from(0.19756347548806534796e1_f64) * t1647 * t20168 - F::cast_from(0.39512695097613069591e1_f64) * t3047 * t23607 - F::cast_from(0.19756347548806534796e1_f64) * t16362 * t6393;
    t80166
}
