//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3098/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3098(t43855: f64, t43859: f64, t43861: f64, t43863: f64, t44249: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64) -> f64 {
    let t64197 = t44249 - 0.3859074074074074074e-1_f64 * t43855 - 0.61745185185185185184e0_f64 * t43859 + 0.11577222222222222222e0_f64 * t43861 + 0.23154444444444444444e0_f64 * t43863 - 0.13772666666666666666e1_f64 * t50903 - 0.68863333333333333332e0_f64 * t50905 - 0.20659e1_f64 * t50907 - 0.6121185185185185185e0_f64 * t50919 - 0.38257407407407407407e0_f64 * t50921 + 0.18363555555555555555e1_f64 * t50948 + 0.45908888888888888888e0_f64 * t50950 + 0.22954444444444444444e0_f64 * t50952 + 0.13772666666666666666e1_f64 * t50954;
    t64197
}
