//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 554/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk554(t1133: f64, t1138: f64, t1141: f64, t1145: f64, t1151: f64, t123: f64, t1316: f64, t1317: f64, t1324: f64, t1772: f64, t1775: f64, t2164: f64, t2276: f64, t2306: f64, t2308: f64, t2312: f64, t2365: f64, t312: f64, t315: f64, t317: f64, t329: f64, t346: f64, t61: f64, t790: f64) -> f64 {
    let t2367 = -t1772 - t1775 - 0.054045904796391424_f64 * t1133 + t1138 - 0.0002905674151788692_f64 * t1141 - t1145 + t1151 + 3.0_f64 * t329 * t2276 + t2306 * t312 - t346 * t2308 * t1324 + 3.0_f64 * t1316 * t2312 + 3.0_f64 * t1316 * t790 * t1317 + 0.020267214298646783_f64 * t123 * t315 * t2164 * t317 + t2365 * t61;
    t2367
}
