//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 892/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk892(t2362: f64, t4502: f64, t3148: f64, t1101: f64, t4411: f64, t4413: f64, t4421: f64, t8595: f64, t8597: f64, t8600: f64, t8602: f64, t8604: f64, t8606: f64, t8608: f64, t9056: f64, t9060: f64, t9394: f64, t9409: f64, t9411: f64, t98: f64) -> f64 {
    let t9414 = t2362 * t4502;
    let t9415 = t9414 * t3148;
    let t9417 = -0.09983749558483038_f64 * t4411 - t4413 / 9.0_f64 + t4421 / 6.0_f64 + t9394 * t98 / 6.0_f64 - 0.02466859483068398_f64 * t8595 - 0.02466859483068398_f64 * t8597 + 0.14975624337724558_f64 * t8600 + 0.29951248675449116_f64 * t8602 + 0.14975624337724558_f64 * t8604 + 0.14975624337724558_f64 * t8606 + 0.29951248675449116_f64 * t8608 + t1101 * t9056 / 6.0_f64 + t1101 * t9060 / 3.0_f64 + t9409 * t9411 / 6.0_f64 + t9415 / 6.0_f64;
    t9417
}
