//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1265/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1265(t297: f64, t301: f64, t413: f64, t7364: f64, t10661: f64, t113: f64, t1316: f64, t14640: f64, t14642: f64, t18883: f64, t18885: f64, t18911: f64, t18915: f64, t18940: f64, t19130: f64, t22214: f64, t22236: f64, t2258: f64, t2308: f64, t2718: f64, t2733: f64, t312: f64, t346: f64, t384: f64, t4398: f64, t4414: f64, t6013: f64, t6024: f64, t7898: f64, t7902: f64, t7906: f64) -> f64 {
    let t22241 = t297 * t7364 * t413 * t301;
    let t22243 = 9.0_f64 * t1316 * t2733 * t4414 + 0.05987117005127304_f64 * t18883 + 0.11974234010254609_f64 * t18885 - 0.9247854820715865_f64 * t10661 - t14640 - 0.002615106736609823_f64 * t14642 - 6.0_f64 * t18940 * t6013 - t346 * t4398 * t7906 - t346 * t2308 * t384 * t2718 + 6.0_f64 * t1316 * t2733 * t6024 - 2.0_f64 * t346 * t4398 * t7902 + 9.0_f64 * t1316 * t2258 * t7898 + 0.5945049527603057_f64 * t18911 + 0.004067943812504169_f64 * t18915 - 0.01197423401025461_f64 * t297 * t19130 * t113 * t301 + (t22214 + t22236) * t312 - 0.01197423401025461_f64 * t22241;
    t22243
}
