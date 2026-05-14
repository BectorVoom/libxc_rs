//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1152/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1152<F: Float>(t1518: F, t2479: F, t548: F, t568: F, t6671: F, t2076: F, t3437: F, t184: F, t509: F, t784: F, t2131: F, t2824: F, t521: F, t1339: F, t35: F, t519: F) -> (F, F, F, F, F, F) {
    let t16961 = t548 * t1518 * t2479;
    let t16962 = 8.0 / 135.0 * t16961;
    let t16963 = t6671 * t568;
    let t16964 = 8.0 / 45.0 * t16963;
    let t16969 = 8.0 / 15.0 * t2076 * t3437;
    let t16971 = t784 * t509 * t184;
    let t16973 = 16.0 / 15.0 * t16971 * t2131;
    let t16974 = t2824 * t521;
    let t16978 = 64.0 / 45.0 * t519 * t16974 * t1339 * t35;
    (t16962, t16964, t16969, t16973, t16974, t16978)
}
