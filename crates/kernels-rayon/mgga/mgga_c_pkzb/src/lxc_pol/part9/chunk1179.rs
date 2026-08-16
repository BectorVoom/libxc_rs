//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1179/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1179(t1045: f64, t1055: f64, t158: f64, t1784: f64, t1791: f64, t1792: f64, t1812: f64, t1813: f64, t184: f64, t188: f64, t20441: f64, t20498: f64, t20553: f64, t2671: f64, t2678: f64, t2679: f64, t2702: f64, t2703: f64, t5408: f64, t5418: f64, t5420: f64, t5462: f64, t626: f64, t7116: f64) -> f64 {
    let t20567 = 0.39512695097613069591e1_f64 * t1784 * t2679 + 0.39512695097613069591e1_f64 * t184 * t7116 * t1812 - 0.19756347548806534796e1_f64 * t2671 * t1813 - 0.11853808529283920877e2_f64 * t184 * t5418 * t2702 * t1791 + 0.65854491829355115987e0_f64 * t20441 * t158 * t188 - 0.19756347548806534796e1_f64 * t1784 * t2703 - 0.65854491829355115987e0_f64 * t184 * t626 * (t20498 + t20553) - 0.65854491829355115987e0_f64 * t5408 * t1055 - 0.39512695097613069591e1_f64 * t1045 * t5420 + 0.13170898365871023197e1_f64 * t184 * t2678 * t5462 + 0.39512695097613069591e1_f64 * t2671 * t1792;
    t20567
}
