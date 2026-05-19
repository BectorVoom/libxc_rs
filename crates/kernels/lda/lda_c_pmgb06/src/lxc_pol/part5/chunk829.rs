//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 829/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk829<F: Float>(t783: F, t73: F, t769: F, t787: F, t2718: F, t6031: F, t790: F, t1316: F, t2180: F, t2308: F, t2741: F, t329: F, t346: F, t3991: F, t3999: F, t4005: F, t5986: F, t5990: F, t6021: F, t7306: F, t77: F) -> (F, F, F, F, F, F, F) {
    let t7881 = t783 * t783;
    let t7882 = t73 * t7881;
    let t7898 = t787 * t769;
    let t7902 = t787 * t783;
    let t7906 = t73 * t2718;
    let t7909 = t790 * t6031;
    let t7912 = F::new(3.0) * t329 * t77 * t7306 - F::cast_from(0.16213771438917426_f64) * t5986 - F::cast_from(0.0008717022455366076_f64) * t5990 - t3991 + t3999 - t4005 - F::new(2.0) * t346 * t6021 * t2741 + F::new(9.0) * t1316 * t790 * t7898 - F::new(2.0) * t346 * t2308 * t7902 - t346 * t2308 * t7906 + F::new(18.0) * t2180 * t7909;
    (t7881, t7882, t7898, t7902, t7906, t7909, t7912)
}
