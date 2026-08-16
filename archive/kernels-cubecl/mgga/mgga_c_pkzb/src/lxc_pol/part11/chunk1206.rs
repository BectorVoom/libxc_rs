//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1206/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1206<F: Float>(t1045: F, t10685: F, t10686: F, t10689: F, t10727: F, t10728: F, t17121: F, t1790: F, t184: F, t19873: F, t2678: F, t2702: F, t29514: F, t29574: F, t29634: F, t3487: F, t622: F, t626: F, t633: F, t7116: F, t9033: F, t9095: F, t9096: F) -> F {
    let t29639 = -F::cast_from(0.19756347548806534796e1_f64) * t1045 * t9096 - F::cast_from(0.39512695097613069591e1_f64) * t622 * t10686 + F::cast_from(0.15805078039045227836e2_f64) * t184 * t17121 * t10685 * t633 - F::cast_from(0.11853808529283920877e2_f64) * t184 * t9033 * t2702 + F::cast_from(0.39512695097613069591e1_f64) * t622 * t10689 - F::cast_from(0.11853808529283920877e2_f64) * t19873 * t29514 * t633 + F::cast_from(0.39512695097613069591e1_f64) * t184 * t7116 * t3487 + F::cast_from(0.39512695097613069591e1_f64) * t184 * t2678 * t9095 - F::cast_from(0.65854491829355115987e0_f64) * t622 * t10728 + F::cast_from(0.13170898365871023197e1_f64) * t184 * t1790 * t10727 * t633 - F::cast_from(0.65854491829355115987e0_f64) * t184 * t626 * (t29574 + t29634);
    t29639
}
