//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1069/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1069(t10004: f64, t11589: f64, t10959: f64, t11066: f64, t11096: f64, t11500: f64, t11502: f64, t11504: f64, t11510: f64, t11512: f64, t11515: f64, t11517: f64, t11520: f64, t11580: f64, t11583: f64, t11587: f64, t2032: f64, t2783: f64, t455: f64, t6594: f64, t6981: f64, t6982: f64, t6984: f64, t6985: f64, t7293: f64, t7304: f64) -> f64 {
    let t11590 = t10004 * t11589;
    let t11594 = t11500 / 18.0_f64 - t11502 / 6.0_f64 - t11504 * t11096 / 3.0_f64 + t7304 * t2783 / 6.0_f64 + t11510 / 6.0_f64 + t11512 * t2032 / 6.0_f64 - t11515 / 6.0_f64 - t11517 * t11096 / 3.0_f64 + t11520 / 18.0_f64 + t7293 * t2783 / 6.0_f64 - 0.10237773105191754_f64 * t11066 - 0.20475546210383508_f64 * t10959 - t11580 * t455 / 6.0_f64 - t11583 * t455 / 6.0_f64 + t6981 + t11587 * t11590 / 3.0_f64 + t6982 + t6984 + t6985 - 0.02466859483068398_f64 * t6594;
    t11594
}
