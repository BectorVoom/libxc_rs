//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1102/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1102(t2740: f64, t6253: f64, t1672: f64, t2943: f64, t2762: f64, t2769: f64, t2765: f64, t12147: f64, t455: f64, t7353: f64, t7395: f64, t7400: f64, t7402: f64, t7411: f64, t7413: f64, t7415: f64, t7418: f64, t7421: f64, t7422: f64, t7426: f64, t7430: f64) -> f64 {
    let t12150 = t2740 * t6253;
    let t12154 = t2943 * t1672;
    let t12156 = t2762 * t1672;
    let t12161 = t2769 * t1672;
    let t12164 = t2765 * t1672;
    let t12169 = -2.2140749178833072_f64 * t12147 * t455 + 0.9941357652469939_f64 * t12150 + 0.8091720650647759_f64 * t7353 + 0.7380249726277691_f64 * t7395 + t7400 - t7402 + 0.7380249726277691_f64 * t12154 - 6.496391258193384_f64 * t12156 - 6.211752672544321_f64 * t7411 - 1.6457779058161184_f64 * t7413 + 0.8091720650647759_f64 * t7415 - 0.6268457032291772_f64 * t12161 + 0.7380249726277691_f64 * t7418 - 1.6457779058161184_f64 * t12164 - t7421 - 3.7610742193750633_f64 * t7422 + 1.8805371096875316_f64 * t7426 - 2.2140749178833072_f64 * t7430;
    t12169
}
