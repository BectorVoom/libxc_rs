//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 970/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk970(t5043: f64, t5056: f64, t5158: f64, t5166: f64, t5177: f64, t5193: f64, t5907: f64, t5923: f64, t9623: f64, t9631: f64, t9635: f64, t9742: f64, t9750: f64, t9948: f64, t9952: f64, t9956: f64, t9959: f64) -> f64 {
    let t10330 = 1.5323028051206833_f64 * t9948 + 1.5323028051206833_f64 * t9952 - 1.5323028051206833_f64 * t9956 + 1.0215352034137888_f64 * t9959 - 0.3056501876701794_f64 * t9623 - 0.1018833958900598_f64 * t9631 - 0.3056501876701794_f64 * t9635 - 0.3056501876701794_f64 * t9742 - 0.3056501876701794_f64 * t9750 - 0.3056501876701794_f64 * t5043 - 0.1018833958900598_f64 * t5056 + t5907 - 1.0215352034137888_f64 * t5177 + 1.0215352034137888_f64 * t5193 + t5923 - 3.0646056102413666_f64 * t5158 + 3.0646056102413666_f64 * t5166;
    t10330
}
