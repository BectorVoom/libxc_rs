//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1032/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1032<F: Float>(t1351: F, t4574: F, t3975: F, t2104: F, t5175: F, t1284: F, t1518: F, t185: F, t2099: F, t4500: F, t784: F, t4521: F, t811: F, t34: F, t10030: F, t4476: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13962 = t4574 * t1351;
    let t13966 = t3975 * t1351;
    let t13975 = t2104 * t5175;
    let t13977 = t1284 * t5175;
    let t14004 = t185 * t1518 * t2099;
    let t14014 = t4500 * t784;
    let t14030 = t4521 * t811;
    let t14034 = t3975 * t34;
    let t14041 = t10030 * t4476;
    (t13962, t13966, t13975, t13977, t14004, t14014, t14030, t14034, t14041)
}
