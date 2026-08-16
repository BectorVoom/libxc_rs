//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1237/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1237(t10657: f64, t10661: f64, t11286: f64, t1227: f64, t1309: f64, t1316: f64, t14633: f64, t14640: f64, t14642: f64, t14646: f64, t14648: f64, t14656: f64, t14694: f64, t14746: f64, t2180: f64, t2258: f64, t2276: f64, t312: f64, t329: f64, t346: f64, t3656: f64, t388: f64, t4358: f64, t4405: f64, t5731: f64, t5903: f64, t77: f64, t790: f64, t8065: f64) -> f64 {
    let t14752 = 18.0_f64 * t2180 * t5731 * t1227 + 3.0_f64 * t346 * t2258 * t1309 - 18.0_f64 * t4358 * t14633 + 0.5945049527603057_f64 * t10657 - 2.7743564462147594_f64 * t10661 - t14640 - 0.0008717022455366076_f64 * t14642 - 0.0008717022455366076_f64 * t14646 + 9.0_f64 * t1316 * t388 * t14648 + t346 * t790 * t3656 + 18.0_f64 * t4405 * t2276 + 3.0_f64 * t1316 * t388 * t14656 + 6.0_f64 * t346 * t5903 * t8065 + (t14694 + t14746) * t312 + 3.0_f64 * t329 * t77 * t11286;
    t14752
}
