//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1180/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1180<F: Float>(t1045: F, t1054: F, t12508: F, t135: F, t144: F, t1535: F, t16701: F, t16873: F, t17000: F, t17121: F, t1790: F, t1812: F, t184: F, t19809: F, t19823: F, t19825: F, t19867: F, t19873: F, t20326: F, t20567: F, t2537: F, t2575: F, t2714: F, t2718: F, t5082: F, t5419: F, t5424: F, t5463: F, t560: F, t622: F, t633: F, t634: F, t639: F, t6763: F, t7097: F, t7113: F, t7117: F, t7120: F, t7173: F, t7174: F) -> F {
    let t20572 = t16873 - F::new(18.0) * t2718 * t2537 * t19809 + F::new(18.0) * t2718 * t2714 * t17000 - F::new(9.0) * t1535 * t5082 * t2575 + F::new(18.0) * t135 * t6763 * t2575 + t16701 - t19823 + t19825 + F::new(3.0) * t135 * t560 * t19867 + t135 * t144 * (F::new(0.79025390195226139182e1) * t622 * t7117 - F::new(0.11853808529283920877e2) * t19873 * t12508 * t1812 + F::new(0.15805078039045227836e2) * t184 * t17121 * t1054 * t5419 - F::new(0.11853808529283920877e2) * t622 * t7113 - F::new(0.65854491829355115987e0) * t1045 * t5463 + F::new(0.39512695097613069591e1) * t1045 * t5424 - F::new(0.19756347548806534796e1) * t7097 * t634 + F::new(0.39512695097613069591e1) * t622 * t7120 - F::new(0.19756347548806534796e1) * t622 * t7174 + F::new(0.39512695097613069591e1) * t184 * t1790 * t7173 * t633 + t20567) * t639 + t20326;
    t20572
}
