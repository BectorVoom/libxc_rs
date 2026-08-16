//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 802/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk802(t161: f64, t164: f64, t3483: f64, t3485: f64, t3488: f64, t3490: f64, t3497: f64, t3500: f64, t3744: f64, t3758: f64, t8046: f64, t8049: f64, t8053: f64, t8061: f64, t8066: f64, t8069: f64, t8073: f64) -> f64 {
    let t8076 = 18.635258017632964_f64 * t8046 + 4.937333717448355_f64 * t161 * t8049 - 0.04115066352984959_f64 * t164 * t8053 + 1.4760499452555382_f64 * t3483 - 12.423505345088643_f64 * t3485 - 12.992782516386768_f64 * t3488 + 1.8805371096875316_f64 * t3490 + t3497 + t3500 - 1.1846959580306418_f64 * t3744 * t8061 + 4.738783832122567_f64 * t8066 + 4.738783832122567_f64 * t3758 * t8069 + 4.738783832122567_f64 * t3758 * t8073;
    t8076
}
