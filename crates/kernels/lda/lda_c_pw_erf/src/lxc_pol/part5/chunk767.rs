//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 767/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk767<F: Float>(t2419: F, t811: F, t1319: F, t1318: F, t2325: F, t5412: F, t1326: F, t1325: F, t2433: F, t806: F, t1313: F, t519: F, t2415: F, t833: F, t1308: F, t571: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7733 = t2419 * t811;
    let t7734 = t1319 * t7733;
    let t7736 = 8.0 / 15.0 * t1318 * t7734;
    let t7737 = t5412 * t2325;
    let t7738 = t1326 * t7737;
    let t7740 = 16.0 / 15.0 * t1325 * t7738;
    let t7741 = t2433 * t806;
    let t7742 = t1313 * t7741;
    let t7744 = 8.0 / 15.0 * t519 * t7742;
    let t7745 = t2415 * t833;
    let t7746 = t1308 * t7745;
    let t7748 = 8.0 / 15.0 * t571 * t7746;
    (t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742, t7744, t7745, t7746, t7748)
}
