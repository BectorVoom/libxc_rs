//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1022/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1022<F: Float>(t13778: F, t3974: F, t5166: F, t12428: F, t1351: F, t13124: F, t4506: F, t12414: F, t4518: F, t1381: F, t352: F, t743: F, t4515: F, t4516: F, t954: F, t3604: F, t4521: F) -> (F, F, F, F, F, F, F, F) {
    let t13796 = 8.0 / 9.0 * t3974 * t5166 * t13778;
    let t13797 = t12428 * t1351;
    let t13800 = 8.0 / 9.0 * t4506 * t13797 * t13124;
    let t13802 = 16.0 / 15.0 * t12414 * t4518;
    let t13804 = t743 * t1381 * t352;
    let t13807 = 8.0 / 15.0 * t4506 * t4515 * t13804;
    let t13808 = t4516 * t954;
    let t13811 = 8.0 / 15.0 * t4506 * t4515 * t13808;
    let t13812 = t4521 * t3604;
    (t13796, t13800, t13802, t13804, t13807, t13808, t13811, t13812)
}
