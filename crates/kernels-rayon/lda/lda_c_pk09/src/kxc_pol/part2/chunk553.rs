//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 553/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk553(t3453: f64, t200: f64, t3230: f64, t3233: f64, t192: f64, t2983: f64, t179: f64, t155: f64, t2974: f64, t3262: f64, t177: f64, t733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3454 = 50.0_f64 * t3453;
    let t3475 = t200 * t3230;
    let t3477 = t200 * t3233;
    let t3483 = t192 * t2983;
    let t3485 = t179 * t2983;
    let t3488 = t155 * t2983;
    let t3490 = t3262 * t2974;
    let t3494 = t177 * t733;
    (t3454, t3475, t3477, t3483, t3485, t3488, t3490, t3494)
}
