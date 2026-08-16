//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 711/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk711(t495: f64, t1745: f64, t429: f64, t1741: f64, t6574: f64, t6580: f64, t6588: f64, t6591: f64, t463: f64, t6601: f64, t453: f64, t6710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6971 = t495 * t495;
    let t6972 = 1.0_f64 / t6971;
    let t6977 = t1745 * t429;
    let t6978 = t1741 * t6977;
    let t6981 = 0.09983749558483038_f64 * t6574;
    let t6982 = 0.1110086767380779_f64 * t6580;
    let t6984 = 0.29951248675449116_f64 * t6588;
    let t6985 = 0.04933718966136796_f64 * t6591;
    let t6989 = 2.0_f64 / 27.0_f64 * t463 * t6601;
    let t6991 = 2.0_f64 / 27.0_f64 * t453 * t6601;
    let t6995 = 0.06655833038988691_f64 * t6710;
    (t6972, t6977, t6978, t6981, t6982, t6984, t6985, t6989, t6991, t6995)
}
