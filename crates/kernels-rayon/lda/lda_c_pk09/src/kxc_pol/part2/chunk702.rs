//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 702/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk702(t4990: f64, t533: f64, t1782: f64, t6579: f64, t1836: f64, t6590: f64, t1853: f64, t6292: f64, t1856: f64, t6477: f64, t1877: f64, t1808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6724 = t533 * t4990;
    let t6725 = t6724 * t1782;
    let t6727 = 21.324527244551554_f64 * t6725 * t6579;
    let t6729 = 9.477567664245134_f64 * t1836 * t6590;
    let t6735 = 2.427516195194328_f64 * t1853 * t6292;
    let t6736 = t1856 * t6477;
    let t6739 = 19.489173774580152_f64 * t1877 * t6292;
    let t6740 = t1808 * t6477;
    (t6727, t6729, t6735, t6736, t6739, t6740)
}
