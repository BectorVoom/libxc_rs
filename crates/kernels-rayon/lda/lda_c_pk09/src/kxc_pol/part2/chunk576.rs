//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 576/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk576(t113: f64, t61: f64, t650: f64, t891: f64, t733: f64, t861: f64, t3743: f64, t62: f64) -> (f64, f64, f64, f64, f64) {
    let t4086 = t61 * t113;
    let t4088 = t891 * t4086 * t650;
    let t4091 = t861 * t733;
    let t4092 = t4091 * t3743;
    let t4093 = t62 * t113;
    (t4086, t4088, t4091, t4092, t4093)
}
