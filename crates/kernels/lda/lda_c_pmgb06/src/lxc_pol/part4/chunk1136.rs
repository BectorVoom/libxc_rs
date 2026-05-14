//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1136/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1136<F: Float>(t13211: F, t13213: F, t13215: F, t13218: F, t4612: F, t6275: F, t13220: F, t2477: F, t3177: F, t2614: F, t955: F, t2617: F, t2620: F, t1414: F, t79: F, t405: F, t6879: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17012 = 4.0 / 135.0 * t13211;
    let t17013 = 8.0 / 135.0 * t13213;
    let t17014 = 4.0 / 135.0 * t13215;
    let t17015 = 16.0 / 135.0 * t13218;
    let t17017 = 8.0 / 45.0 * t6275 * t4612;
    let t17018 = 4.0 / 135.0 * t13220;
    let t17020 = 2.0 / 45.0 * t3177 * t2477;
    let t17025 = t955 * t2614;
    let t17030 = t955 * t2617;
    let t17035 = t955 * t2620;
    let t17037 = t1414 * t79;
    let t17041 = t405 * t6879;
    (t17012, t17013, t17014, t17015, t17017, t17018, t17020, t17025, t17030, t17035, t17037, t17041)
}
