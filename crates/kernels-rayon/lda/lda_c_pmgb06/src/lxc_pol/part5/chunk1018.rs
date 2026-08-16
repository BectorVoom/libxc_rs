//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1018/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1018(t1447: f64, t7715: f64, t1420: f64, t15180: f64, t15182: f64, t15184: f64, t15189: f64, t2010: f64, t2948: f64, t439: f64, t5482: f64, t6146: f64, t6185: f64, t6189: f64, t6375: f64, t6494: f64, t6498: f64, t7554: f64, t7555: f64) -> f64 {
    let t19178 = t1447 * t7715;
    let t19181 = 2.0_f64 / 15.0_f64 * t439 * t5482 * t6375 + 2.0_f64 / 5.0_f64 * t439 * t6494 * t6185 - 2.0_f64 / 3.0_f64 * t439 * t6498 * t6146 - 8.0_f64 / 15.0_f64 * t2010 * t6494 * t6189 - t1420 * t7555 / 15.0_f64 - t439 * t2948 * t7554 / 15.0_f64 - 4.0_f64 / 45.0_f64 * t15180 - 8.0_f64 / 45.0_f64 * t15182 - 4.0_f64 / 45.0_f64 * t15184 + 4.0_f64 / 45.0_f64 * t19178 - 2.0_f64 / 45.0_f64 * t15189;
    t19181
}
