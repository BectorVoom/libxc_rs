//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 880/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk880(t1058: f64, t8720: f64, t1062: f64, t2379: f64, t721: f64, t1007: f64, t1011: f64, t2337: f64, t2341: f64, t4021: f64, t4353: f64, t4354: f64, t4362: f64, t4366: f64, t8508: f64, t8510: f64, t8512: f64, t8517: f64, t8519: f64, t8521: f64, t98: f64) -> f64 {
    let t9220 = t8720 * t1058;
    let t9223 = t2379 * t1062;
    let t9224 = t9223 * t721;
    let t9237 = -t1007 * t2341 / 6.0_f64 - t9220 * t98 / 6.0_f64 + t9224 / 6.0_f64 + t2337 * t1011 / 6.0_f64 + t4353 + t4354 - 0.016445729887122652_f64 * t4021 + t4362 / 6.0_f64 + t4366 / 6.0_f64 + 0.037002892246025966_f64 * t8508 - 0.02466859483068398_f64 * t8510 - 0.02466859483068398_f64 * t8512 + 0.02466859483068398_f64 * t8517 + 0.02466859483068398_f64 * t8519 + 0.14975624337724558_f64 * t8521;
    t9237
}
