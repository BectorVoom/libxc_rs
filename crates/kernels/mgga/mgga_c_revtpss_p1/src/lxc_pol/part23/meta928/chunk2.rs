//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3023/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3023<F: Float>(t1024: F, t1093: F, t12160: F, t15670: F, t1647: F, t16509: F, t19399: F, t19438: F, t19443: F, t19463: F, t19539: F, t19566: F, t19617: F, t20113: F, t23598: F, t23959: F, t24152: F, t3204: F, t3291: F, t3316: F, t342: F, t381: F, t4857: F, t4988: F, t4999: F, t5004: F, t5005: F, t6343: F, t79388: F) -> F {
    let t80490 = F::cast_from(0.65854491829355115987e0_f64) * t79388 * t381 + F::cast_from(0.65854491829355115987e0_f64) * t23959 * t1093 + F::cast_from(0.39512695097613069591e1_f64) * t16509 * t19539 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t3291 * t23598 - F::cast_from(0.19756347548806534796e1_f64) * t19463 * t5005 - F::cast_from(0.19756347548806534796e1_f64) * t4857 * t19443 + F::cast_from(0.79025390195226139182e1_f64) * t3204 * t5004 * t19399 + F::cast_from(0.19756347548806534796e1_f64) * t19566 * t4988 - F::cast_from(0.19756347548806534796e1_f64) * t4857 * t19438 - F::cast_from(0.19756347548806534796e1_f64) * t12160 * t24152 + F::cast_from(0.39512695097613069592e1_f64) * t15670 * t19617 + F::cast_from(0.19756347548806534796e1_f64) * t1647 * t20113 - F::cast_from(0.19756347548806534796e1_f64) * t342 * t3316 * t6343 * t4999;
    t80490
}
