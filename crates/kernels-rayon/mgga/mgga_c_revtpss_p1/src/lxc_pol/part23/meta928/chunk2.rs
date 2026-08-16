//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3023/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3023(t1024: f64, t1093: f64, t12160: f64, t15670: f64, t1647: f64, t16509: f64, t19399: f64, t19438: f64, t19443: f64, t19463: f64, t19539: f64, t19566: f64, t19617: f64, t20113: f64, t23598: f64, t23959: f64, t24152: f64, t3204: f64, t3291: f64, t3316: f64, t342: f64, t381: f64, t4857: f64, t4988: f64, t4999: f64, t5004: f64, t5005: f64, t6343: f64, t79388: f64) -> f64 {
    let t80490 = 0.65854491829355115987e0_f64 * t79388 * t381 + 0.65854491829355115987e0_f64 * t23959 * t1093 + 0.39512695097613069591e1_f64 * t16509 * t19539 - 0.65854491829355115987e0_f64 * t1024 * t3291 * t23598 - 0.19756347548806534796e1_f64 * t19463 * t5005 - 0.19756347548806534796e1_f64 * t4857 * t19443 + 0.79025390195226139182e1_f64 * t3204 * t5004 * t19399 + 0.19756347548806534796e1_f64 * t19566 * t4988 - 0.19756347548806534796e1_f64 * t4857 * t19438 - 0.19756347548806534796e1_f64 * t12160 * t24152 + 0.39512695097613069592e1_f64 * t15670 * t19617 + 0.19756347548806534796e1_f64 * t1647 * t20113 - 0.19756347548806534796e1_f64 * t342 * t3316 * t6343 * t4999;
    t80490
}
