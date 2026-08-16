//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 825/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk825(t1125: f64, t3744: f64, t3758: f64, t3775: f64, t3778: f64, t3821: f64, t3844: f64, t8315: f64, t8318: f64, t8322: f64, t8326: f64, t8331: f64, t8334: f64, t8338: f64, t8342: f64, t8345: f64) -> f64 {
    let t8347 = -2.427516195194328_f64 * t3775 + 2.427516195194328_f64 * t3778 - 3.7610742193750633_f64 * t3821 - 1.8805371096875316_f64 * t3844 - t1125 * t8315 - 4.738783832122567_f64 * t3758 * t8318 + 4.738783832122567_f64 * t3758 * t8322 + 1.1846959580306418_f64 * t3744 * t8326 - 4.738783832122567_f64 * t8331 - 4.738783832122567_f64 * t3758 * t8334 - 7.108175748183851_f64 * t3758 * t8338 + 4.738783832122567_f64 * t3758 * t8342 - 1.2536914064583544_f64 * t8345;
    t8347
}
