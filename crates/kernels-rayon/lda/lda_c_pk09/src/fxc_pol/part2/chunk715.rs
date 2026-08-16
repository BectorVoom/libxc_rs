//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 715/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk715(t1672: f64, t2104: f64, t472: f64, t6601: f64, t2000: f64, t451: f64, t6196: f64, t6501: f64, t6505: f64, t6522: f64, t6319: f64, t6325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7091 = t2104 * t1672;
    let t7098 = 2.0_f64 / 27.0_f64 * t472 * t6601;
    let t7102 = t451 * t2000;
    let t7103 = t7102 * t6196;
    let t7107 = 1.5323028051206833_f64 * t6501;
    let t7108 = 1.5323028051206833_f64 * t6505;
    let t7112 = 2.0430704068275776_f64 * t6522;
    let t7116 = 0.3056501876701794_f64 * t6319;
    let t7123 = 0.2037667917801196_f64 * t6325;
    (t7091, t7098, t7102, t7103, t7107, t7108, t7112, t7116, t7123)
}
