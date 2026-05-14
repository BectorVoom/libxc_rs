//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1146/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1146<F: Float>(t22812: F, t22813: F, t22814: F, t22815: F, t22816: F, t22821: F, t22824: F, t22826: F, t22828: F, t22830: F, t22833: F, t22836: F, t22839: F, t1: F, t3: F, t604: F, t7337: F) -> (F, F) {
    let t23315 = -t22812 - t22813 - t22814 - t22815 + t22816 + t22821 + t22824 - t22826 + t22828 + t22830 + t22833 - t22836 + t22839;
    let t23321 = t7337 * t1 * t3 * t604;
    (t23315, t23321)
}
