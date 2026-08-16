//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 829/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk829(t783: f64, t73: f64, t769: f64, t787: f64, t2718: f64, t6031: f64, t790: f64, t1316: f64, t2180: f64, t2308: f64, t2741: f64, t329: f64, t346: f64, t3991: f64, t3999: f64, t4005: f64, t5986: f64, t5990: f64, t6021: f64, t7306: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7881 = t783 * t783;
    let t7882 = t73 * t7881;
    let t7898 = t787 * t769;
    let t7902 = t787 * t783;
    let t7906 = t73 * t2718;
    let t7909 = t790 * t6031;
    let t7912 = 3.0_f64 * t329 * t77 * t7306 - 0.16213771438917426_f64 * t5986 - 0.0008717022455366076_f64 * t5990 - t3991 + t3999 - t4005 - 2.0_f64 * t346 * t6021 * t2741 + 9.0_f64 * t1316 * t790 * t7898 - 2.0_f64 * t346 * t2308 * t7902 - t346 * t2308 * t7906 + 18.0_f64 * t2180 * t7909;
    (t7881, t7882, t7898, t7902, t7906, t7909, t7912)
}
