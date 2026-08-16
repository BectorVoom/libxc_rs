//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3131/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3131<F: Float>(t1887: F, t337: F, t5416: F, t3447: F, t4904: F, t51968: F, t11575: F, t15376: F, t15409: F, t15412: F, t18427: F, t3452: F, t4900: F, t4908: F, t52096: F, t63315: F, t63368: F, t63390: F, t63402: F, t63406: F, t63410: F, t63420: F) -> F {
    let t64811 = t5416 * t337 * t1887;
    let t64821 = t3447 * t51968 * t4904;
    let t64823 = -F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4908 * t63410 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t11575 * t18427 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4908 * t63402 - F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t4908 * t63406 + F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t4900 * t63315 + F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t4900 * t63368 - F::cast_from(0.19753086419753086419e-2_f64) * t15376 * t15409 - F::cast_from(0.11851851851851851851e-1_f64) * t15376 * t15412 + F::cast_from(0.54320987654320987654e-2_f64) * t64811 * t3452 + F::cast_from(0.13333333333333333332e-1_f64) * t3447 * t4900 * t63390 + F::cast_from(0.28806584362139917695e-2_f64) * t3447 * t52096 * t63420 - F::cast_from(0.12345679012345679012e-3_f64) * t64821;
    t64823
}
