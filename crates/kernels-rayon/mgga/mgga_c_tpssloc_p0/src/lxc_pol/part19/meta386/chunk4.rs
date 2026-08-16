//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1451/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1451(t11546: f64, t11571: f64, t11575: f64, t11579: f64, t11584: f64, t11593: f64, t1174: f64, t3440: f64, t3441: f64, t3447: f64, t39097: f64, t39103: f64, t43715: f64, t44558: f64, t44564: f64, t44566: f64, t44573: f64, t44581: f64, t44586: f64, t44589: f64, t44592: f64, t44595: f64, t4900: f64) -> f64 {
    let t44600 = 0.16666666666666666666e-2_f64 * t3447 * t11575 * t11579 + 0.33333333333333333332e-2_f64 * t3447 * t11575 * t11584 + 0.16666666666666666666e-2_f64 * t3447 * t11593 * t11579 - 0.22222222222222222222e-2_f64 * t3447 * t44558 * t11571 - 0.11522633744855967078e-2_f64 * t44564 - 0.1037037037037037037e-1_f64 * t1174 * t11546 * t44566 * t39097 - 0.49382716049382716048e-3_f64 * t44573 + 0.11111111111111111111e-2_f64 * t1174 * t3440 * t3441 * t39103 + 0.11111111111111111111e-2_f64 * t44581 - 0.74074074074074074072e-3_f64 * t44586 + 0.11111111111111111111e-2_f64 * t44589 - 0.22222222222222222221e-2_f64 * t44592 + 0.14814814814814814815e-2_f64 * t44595 + 0.14814814814814814815e-2_f64 * t3447 * t4900 * t43715;
    t44600
}
