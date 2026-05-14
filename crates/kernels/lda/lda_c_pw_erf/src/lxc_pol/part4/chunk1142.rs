//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1142/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1142<F: Float>(t16031: F, t348: F, t12362: F, t4494: F, t4501: F, t16595: F, t4488: F, t12380: F, t16616: F, t1287: F, t3974: F, t3976: F, t6723: F, t12064: F, t6730: F, t2325: F, t494: F, t542: F) -> (F, F, F, F, F, F, F) {
    let t16802 = t16031 * t348;
    let t16805 = 64.0 / 45.0 * t12362 * t4494 * t16802;
    let t16808 = 32.0 / 27.0 * t12362 * t4501 * t16802;
    let t16811 = 8.0 / 27.0 * t4488 * t4501 * t16595;
    let t16814 = 64.0 / 81.0 * t4488 * t12380 * t16616;
    let t16818 = 16.0 / 45.0 * t3974 * t3976 * t6723 * t1287;
    let t16819 = t12064 * t6730;
    let t16820 = 64.0 / 135.0 * t16819;
    let t16822 = t2325 * t494 * t542;
    (t16805, t16808, t16811, t16814, t16818, t16820, t16822)
}
