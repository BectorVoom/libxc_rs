//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 736/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk736(t3262: f64, t7608: f64, t2210: f64, t2214: f64, t2975: f64, t2981: f64, t3265: f64, t3268: f64, t3826: f64, t3829: f64, t7578: f64, t7584: f64, t7586: f64, t7590: f64, t7598: f64, t7602: f64) -> f64 {
    let t7609 = t3262 * t7608;
    let t7611 = 4.937333717448355_f64 * t2975 - 4.937333717448355_f64 * t2981 + 3.7610742193750633_f64 * t3265 * t7578 - 1.8805371096875316_f64 * t3268 * t2214 - 1.8805371096875316_f64 * t7584 * t7586 + 19.489173774580152_f64 * t3826 * t7590 + 38.978347549160304_f64 * t3826 * t7578 - 19.489173774580152_f64 * t3829 * t2214 + 3.7610742193750633_f64 * t3265 * t7598 + 1.8805371096875316_f64 * t3265 * t7602 - 1.8805371096875316_f64 * t3268 * t2210 + 1.8805371096875316_f64 * t7609;
    t7611
}
