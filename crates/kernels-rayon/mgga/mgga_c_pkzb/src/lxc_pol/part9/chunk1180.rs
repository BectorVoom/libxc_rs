//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1180/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1180(t1045: f64, t1054: f64, t12508: f64, t135: f64, t144: f64, t1535: f64, t16701: f64, t16873: f64, t17000: f64, t17121: f64, t1790: f64, t1812: f64, t184: f64, t19809: f64, t19823: f64, t19825: f64, t19867: f64, t19873: f64, t20326: f64, t20567: f64, t2537: f64, t2575: f64, t2714: f64, t2718: f64, t5082: f64, t5419: f64, t5424: f64, t5463: f64, t560: f64, t622: f64, t633: f64, t634: f64, t639: f64, t6763: f64, t7097: f64, t7113: f64, t7117: f64, t7120: f64, t7173: f64, t7174: f64) -> f64 {
    let t20572 = t16873 - 18.0_f64 * t2718 * t2537 * t19809 + 18.0_f64 * t2718 * t2714 * t17000 - 9.0_f64 * t1535 * t5082 * t2575 + 18.0_f64 * t135 * t6763 * t2575 + t16701 - t19823 + t19825 + 3.0_f64 * t135 * t560 * t19867 + t135 * t144 * (0.79025390195226139182e1_f64 * t622 * t7117 - 0.11853808529283920877e2_f64 * t19873 * t12508 * t1812 + 0.15805078039045227836e2_f64 * t184 * t17121 * t1054 * t5419 - 0.11853808529283920877e2_f64 * t622 * t7113 - 0.65854491829355115987e0_f64 * t1045 * t5463 + 0.39512695097613069591e1_f64 * t1045 * t5424 - 0.19756347548806534796e1_f64 * t7097 * t634 + 0.39512695097613069591e1_f64 * t622 * t7120 - 0.19756347548806534796e1_f64 * t622 * t7174 + 0.39512695097613069591e1_f64 * t184 * t1790 * t7173 * t633 + t20567) * t639 + t20326;
    t20572
}
