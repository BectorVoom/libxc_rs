//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1206/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1206(t1045: f64, t10685: f64, t10686: f64, t10689: f64, t10727: f64, t10728: f64, t17121: f64, t1790: f64, t184: f64, t19873: f64, t2678: f64, t2702: f64, t29514: f64, t29574: f64, t29634: f64, t3487: f64, t622: f64, t626: f64, t633: f64, t7116: f64, t9033: f64, t9095: f64, t9096: f64) -> f64 {
    let t29639 = -0.19756347548806534796e1_f64 * t1045 * t9096 - 0.39512695097613069591e1_f64 * t622 * t10686 + 0.15805078039045227836e2_f64 * t184 * t17121 * t10685 * t633 - 0.11853808529283920877e2_f64 * t184 * t9033 * t2702 + 0.39512695097613069591e1_f64 * t622 * t10689 - 0.11853808529283920877e2_f64 * t19873 * t29514 * t633 + 0.39512695097613069591e1_f64 * t184 * t7116 * t3487 + 0.39512695097613069591e1_f64 * t184 * t2678 * t9095 - 0.65854491829355115987e0_f64 * t622 * t10728 + 0.13170898365871023197e1_f64 * t184 * t1790 * t10727 * t633 - 0.65854491829355115987e0_f64 * t184 * t626 * (t29574 + t29634);
    t29639
}
