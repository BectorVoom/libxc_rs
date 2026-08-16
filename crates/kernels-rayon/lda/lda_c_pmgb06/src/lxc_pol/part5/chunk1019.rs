//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1019/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1019(t1: f64, t10152: f64, t10288: f64, t1444: f64, t15196: f64, t15208: f64, t15216: f64, t15237: f64, t15244: f64, t15248: f64, t1972: f64, t2010: f64, t2864: f64, t439: f64, t493: f64, t6399: f64, t6403: f64, t6522: f64, t7558: f64, t7559: f64, t7562: f64) -> f64 {
    let t19204 = -4.0_f64 / 45.0_f64 * t15196 + 4.0_f64 / 45.0_f64 * t15208 + 4.0_f64 / 45.0_f64 * t15216 - 2.0_f64 / 45.0_f64 * t15237 - 4.0_f64 / 45.0_f64 * t15244 - 4.0_f64 / 45.0_f64 * t15248 + 2.0_f64 / 15.0_f64 * t439 * t10288 * t7562 + 4.0_f64 / 15.0_f64 * t2010 * t2864 * t6522 * t1 + 2.0_f64 / 15.0_f64 * t1972 * t6399 + 2.0_f64 / 5.0_f64 * t1972 * t6403 + 2.0_f64 / 15.0_f64 * t1444 * t7559 + 2.0_f64 / 15.0_f64 * t493 * t10152 * t7558;
    t19204
}
