//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1192/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1192<F: Float>(t13238: F, t13241: F, t13244: F, t13252: F, t13255: F, t17585: F, t17587: F, t17589: F, t17592: F, t17595: F, t17596: F, t17597: F, t17598: F, t17599: F, t17600: F, t17601: F) -> (F, F, F, F, F, F) {
    let t17602 = 32.0 / 135.0 * t13238;
    let t17603 = 16.0 / 135.0 * t13241;
    let t17604 = 128.0 / 135.0 * t13244;
    let t17605 = 32.0 / 135.0 * t13252;
    let t17606 = 32.0 / 135.0 * t13255;
    let t17607 = t17585 - t17587 - t17589 + t17592 - t17595 + t17596 - t17597 - t17598 + t17599 + t17600 - t17601 - t17602 - t17603 - t17604 - t17605 - t17606;
    (t17602, t17603, t17604, t17605, t17606, t17607)
}
