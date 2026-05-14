//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1322/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1322<F: Float>(t17784: F, t17786: F, t17789: F, t17790: F, t17791: F, t17792: F, t17793: F, t17795: F, t17799: F, t17800: F, t17801: F, t17802: F, t17803: F, t17806: F, t17807: F, t17808: F, t17809: F) -> (F,) {
    let t19272 = t17784 + t17786 + t17789 + t17790 - t17791 - t17792 + t17793 - t17795 + t17799 - t17800 - t17801 - t17802 - t17803 + t17806 - t17807 - t17808 + t17809;
    (t19272,)
}
