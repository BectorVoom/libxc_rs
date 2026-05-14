//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 965/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk965<F: Float>(t20373: F, t156: F, t426: F, t7897: F, t14655: F, t10: F, t14796: F, t1832: F, t1856: F, t20359: F, t20371: F, t2610: F, t2624: F, t411: F, t6121: F, t767: F, t7927: F, t7930: F) -> (F, F, F) {
    let t20374 = 2.923025 * t20373;
    let t20376 = t426 * t156 * t7897;
    let t20390 = 5.84605 * t14655;
    let t20391 = 0.73452 * t20359 + 30.0 * t426 * t10 * t7930 * t411 - 18.0 * t426 * t10 * t2624 * t1832 - 8.81424 * t20371 - t20374 - 3.0 / 2.0 * t20376 + 9.0 / 2.0 * t426 * t10 * t1856 * t2610 + 9.0 / 2.0 * t426 * t10 * t767 * t6121 + 3.0 / 2.0 * t426 * t10 * t7927 * t411 + t20390 - t14796;
    (t20374, t20390, t20391)
}
