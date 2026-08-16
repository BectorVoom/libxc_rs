//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3131/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3131(t1887: f64, t337: f64, t5416: f64, t3447: f64, t4904: f64, t51968: f64, t11575: f64, t15376: f64, t15409: f64, t15412: f64, t18427: f64, t3452: f64, t4900: f64, t4908: f64, t52096: f64, t63315: f64, t63368: f64, t63390: f64, t63402: f64, t63406: f64, t63410: f64, t63420: f64) -> f64 {
    let t64811 = t5416 * t337 * t1887;
    let t64821 = t3447 * t51968 * t4904;
    let t64823 = -0.11111111111111111111e-2_f64 * t3447 * t4908 * t63410 + 0.55555555555555555554e-3_f64 * t3447 * t11575 * t18427 - 0.16666666666666666666e-2_f64 * t3447 * t4908 * t63402 - 0.66666666666666666664e-2_f64 * t3447 * t4908 * t63406 + 0.37037037037037037036e-3_f64 * t3447 * t4900 * t63315 + 0.22222222222222222221e-2_f64 * t3447 * t4900 * t63368 - 0.19753086419753086419e-2_f64 * t15376 * t15409 - 0.11851851851851851851e-1_f64 * t15376 * t15412 + 0.54320987654320987654e-2_f64 * t64811 * t3452 + 0.13333333333333333332e-1_f64 * t3447 * t4900 * t63390 + 0.28806584362139917695e-2_f64 * t3447 * t52096 * t63420 - 0.12345679012345679012e-3_f64 * t64821;
    t64823
}
