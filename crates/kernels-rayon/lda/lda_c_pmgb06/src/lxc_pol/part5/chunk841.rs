//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 841/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk841(t1316: f64, t2180: f64, t2311: f64, t2730: f64, t2733: f64, t312: f64, t346: f64, t4027: f64, t5569: f64, t5573: f64, t5580: f64, t5583: f64, t5591: f64, t5593: f64, t5896: f64, t5901: f64, t5903: f64, t5937: f64, t61: f64, t6958: f64, t7102: f64, t73: f64, t7354: f64, t7428: f64, t7441: f64, t787: f64, t7878: f64, t7882: f64, t790: f64, t7912: f64, t7917: f64, t7921: f64, t8017: f64) -> f64 {
    let t8019 = 2.0_f64 * t346 * t2733 * t787 + t346 * t7354 * t73 + t346 * t790 * t2730 + t7428 * t312 + 0.05987117005127304_f64 * t5937 + (t7441 + t7878) * t61 + 2.0_f64 * t346 * t5903 * t7882 + 0.004067943812504169_f64 * t5569 + 0.5945049527603057_f64 * t5573 - 0.0017434044910732151_f64 * t5580 + t7912 + 0.11974234010254609_f64 * t5591 - 0.15965645347006147_f64 * t5593 - 0.03592270203076383_f64 * t6958 + 18.0_f64 * t2180 * t7917 - t4027 - 9.0_f64 * t5583 * t7921 - 0.03592270203076383_f64 * t5896 - 5.4655730795145296e-05_f64 * t5901 + 9.0_f64 * t1316 * t2733 * t2311 + 9.0_f64 * t1316 * t790 * t7102 + t8017;
    t8019
}
