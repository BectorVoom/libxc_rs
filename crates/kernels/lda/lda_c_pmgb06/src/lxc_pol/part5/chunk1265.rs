//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1265/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1265<F: Float>(t297: F, t301: F, t413: F, t7364: F, t10661: F, t113: F, t1316: F, t14640: F, t14642: F, t18883: F, t18885: F, t18911: F, t18915: F, t18940: F, t19130: F, t22214: F, t22236: F, t2258: F, t2308: F, t2718: F, t2733: F, t312: F, t346: F, t384: F, t4398: F, t4414: F, t6013: F, t6024: F, t7898: F, t7902: F, t7906: F) -> F {
    let t22241 = t297 * t7364 * t413 * t301;
    let t22243 = F::cast_from(9.0_f64) * t1316 * t2733 * t4414 + F::cast_from(0.05987117005127304_f64) * t18883 + F::cast_from(0.11974234010254609_f64) * t18885 - F::cast_from(0.9247854820715865_f64) * t10661 - t14640 - F::cast_from(0.002615106736609823_f64) * t14642 - F::cast_from(6.0_f64) * t18940 * t6013 - t346 * t4398 * t7906 - t346 * t2308 * t384 * t2718 + F::cast_from(6.0_f64) * t1316 * t2733 * t6024 - F::cast_from(2.0_f64) * t346 * t4398 * t7902 + F::cast_from(9.0_f64) * t1316 * t2258 * t7898 + F::cast_from(0.5945049527603057_f64) * t18911 + F::cast_from(0.004067943812504169_f64) * t18915 - F::cast_from(0.01197423401025461_f64) * t297 * t19130 * t113 * t301 + (t22214 + t22236) * t312 - F::cast_from(0.01197423401025461_f64) * t22241;
    t22243
}
