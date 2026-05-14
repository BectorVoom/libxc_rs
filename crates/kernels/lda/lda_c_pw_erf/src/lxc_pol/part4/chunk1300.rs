//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1300/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1300<F: Float>(t16781: F, t16783: F, t16785: F, t16786: F, t16787: F, t16788: F, t16789: F, t16790: F, t16792: F, t16794: F, t16798: F, t16801: F, t16805: F, t16808: F, t16811: F, t16814: F, t16818: F) -> (F,) {
    let t19193 = t16781 - t16783 + t16785 + t16786 + t16787 + t16788 + t16789 - t16790 - t16792 + t16794 - t16798 + t16801 + t16805 - t16808 - t16811 - t16814 - t16818;
    (t19193,)
}
