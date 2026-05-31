//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 841/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk841<F: Float>(t1316: F, t2180: F, t2311: F, t2730: F, t2733: F, t312: F, t346: F, t4027: F, t5569: F, t5573: F, t5580: F, t5583: F, t5591: F, t5593: F, t5896: F, t5901: F, t5903: F, t5937: F, t61: F, t6958: F, t7102: F, t73: F, t7354: F, t7428: F, t7441: F, t787: F, t7878: F, t7882: F, t790: F, t7912: F, t7917: F, t7921: F, t8017: F) -> F {
    let t8019 = F::cast_from(2.0_f64) * t346 * t2733 * t787 + t346 * t7354 * t73 + t346 * t790 * t2730 + t7428 * t312 + F::cast_from(0.05987117005127304_f64) * t5937 + (t7441 + t7878) * t61 + F::cast_from(2.0_f64) * t346 * t5903 * t7882 + F::cast_from(0.004067943812504169_f64) * t5569 + F::cast_from(0.5945049527603057_f64) * t5573 - F::cast_from(0.0017434044910732151_f64) * t5580 + t7912 + F::cast_from(0.11974234010254609_f64) * t5591 - F::cast_from(0.15965645347006147_f64) * t5593 - F::cast_from(0.03592270203076383_f64) * t6958 + F::cast_from(18.0_f64) * t2180 * t7917 - t4027 - F::cast_from(9.0_f64) * t5583 * t7921 - F::cast_from(0.03592270203076383_f64) * t5896 - F::cast_from(5.4655730795145296e-05_f64) * t5901 + F::cast_from(9.0_f64) * t1316 * t2733 * t2311 + F::cast_from(9.0_f64) * t1316 * t790 * t7102 + t8017;
    t8019
}
