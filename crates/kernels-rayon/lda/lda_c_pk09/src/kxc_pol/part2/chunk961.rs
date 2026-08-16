//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 961/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk961(t2595: f64, t5819: f64, t1435: f64, t2571: f64, t10162: f64, t1451: f64, t2596: f64, t5404: f64, t5632: f64, t5783: f64, t9623: f64, t9631: f64, t9635: f64, t9742: f64, t9750: f64) -> f64 {
    let t10164 = t2595 * t5819;
    let t10174 = t2571 * t1435;
    let t10177 = t10162 / 18.0_f64 - t10164 * t1451 / 6.0_f64 - t2596 * t5632 / 6.0_f64 + 0.10237773105191754_f64 * t9623 + 0.03412591035063918_f64 * t9631 + 0.10237773105191754_f64 * t9635 + 0.10237773105191754_f64 * t9742 + 0.10237773105191754_f64 * t9750 + t10174 / 18.0_f64 + 0.04991874779241519_f64 * t5404 - t5783;
    t10177
}
