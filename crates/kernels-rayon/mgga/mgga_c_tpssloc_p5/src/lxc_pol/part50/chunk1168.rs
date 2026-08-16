//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1168/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1168(t1888: f64, t23270: f64, t25170: f64, t112678: f64, t112676: f64, t118476: f64, t118479: f64, t118481: f64, t118484: f64, t118488: f64, t118491: f64, t118498: f64, t118499: f64, t118500: f64, t13463: f64, t25168: f64, t25188: f64, t40889: f64, t4272: f64, t6632: f64, t8352: f64, t8353: f64) -> f64 {
    let t118503 = 0.9869604401089358619e-1_f64 * t1888 * t23270 * t25170;
    let t118506 = 0.82246703342411321825e-2_f64 * t112678;
    let t118509 = 24.0_f64 * t25168 * t40889 * t4272 * t8352 + 2.0_f64 * t13463 * t8353 + 4.0_f64 * t25188 * t6632 - t112676 + t118476 + t118479 - t118481 + t118484 - t118488 + t118491 + t118498 + t118499 + t118500 - t118503 + t118506;
    t118509
}
