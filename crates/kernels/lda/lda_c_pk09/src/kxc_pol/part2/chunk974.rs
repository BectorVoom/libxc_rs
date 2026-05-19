//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 974/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk974<F: Float>(t10369: F, t10385: F, t300: F, t306: F, t3248: F, t6033: F, t9920: F, t6037: F, t9748: F, t10346: F, t10349: F, t10352: F, t10355: F, t1609: F, t311: F, t6028: F, t6031: F, t6035: F, t6043: F) -> (F, F, F) {
    let t10386 = t10369 + t10385;
    let t10387 = t300 * t10386;
    let t10388 = t10387 * t306;
    let t10392 = t6033 * t3248 * t9920;
    let t10395 = t6037 * t3248 * t9748;
    let t10401 = t1609 * t10346 / F::new(12.0) + t10349 * t311 / F::new(6.0) + t10352 * t311 / F::new(6.0) + t10355 / F::new(18.0) - t10388 * t311 / F::new(6.0) + F::cast_from(0.07400578449205193_f64) * t10392 - F::cast_from(0.07400578449205193_f64) * t10395 + t6028 / F::new(6.0) + F::cast_from(0.14975624337724558_f64) * t6031 + F::cast_from(0.07400578449205193_f64) * t6035 - F::cast_from(0.07400578449205193_f64) * t6043;
    (t10392, t10395, t10401)
}
