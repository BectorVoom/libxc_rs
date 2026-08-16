//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 514/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk514(t200: f64, t2983: f64, t242: f64, t48: f64, t56: f64, t623: f64, t92: f64, t44: f64, t618: f64, t54: f64, t633: f64, t51: f64, t628: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2984 = t200 * t2983;
    let t2988 = 2.0_f64 / 9.0_f64 * t56 * t242 * t48;
    let t2990 = t56 * t92 * t623;
    let t2993 = 1.0_f64 / t618 / t44;
    let t3007 = 2.0_f64 / 9.0_f64 * t56 * t242 * t54;
    let t3009 = t56 * t92 * t633;
    let t3012 = 1.0_f64 / t628 / t51;
    (t2984, t2988, t2990, t2993, t3007, t3009, t3012)
}
