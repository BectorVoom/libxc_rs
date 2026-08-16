//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 974/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk974(t10369: f64, t10385: f64, t300: f64, t306: f64, t3248: f64, t6033: f64, t9920: f64, t6037: f64, t9748: f64, t10346: f64, t10349: f64, t10352: f64, t10355: f64, t1609: f64, t311: f64, t6028: f64, t6031: f64, t6035: f64, t6043: f64) -> (f64, f64, f64) {
    let t10386 = t10369 + t10385;
    let t10387 = t300 * t10386;
    let t10388 = t10387 * t306;
    let t10392 = t6033 * t3248 * t9920;
    let t10395 = t6037 * t3248 * t9748;
    let t10401 = t1609 * t10346 / 12.0_f64 + t10349 * t311 / 6.0_f64 + t10352 * t311 / 6.0_f64 + t10355 / 18.0_f64 - t10388 * t311 / 6.0_f64 + 0.07400578449205193_f64 * t10392 - 0.07400578449205193_f64 * t10395 + t6028 / 6.0_f64 + 0.14975624337724558_f64 * t6031 + 0.07400578449205193_f64 * t6035 - 0.07400578449205193_f64 * t6043;
    (t10392, t10395, t10401)
}
