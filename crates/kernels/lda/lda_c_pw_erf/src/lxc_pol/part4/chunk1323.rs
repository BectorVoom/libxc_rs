//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1323/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1323<F: Float>(t11153: F, t11156: F, t11159: F, t11162: F, t11164: F, t11166: F, t11168: F, t17810: F, t17811: F, t17812: F, t17813: F, t17823: F, t17825: F, t17827: F, t17830: F, t17833: F, t17836: F) -> (F,) {
    let t19277 = t17810 + t17811 - t17812 - t11153 - t11156 + t11159 + 4.0 / 9.0 * t11162 - 2.0 / 27.0 * t11164 - 4.0 / 9.0 * t11166 - 0.027012345679012346 * t11168 - t17813 - t17823 - t17825 + t17827 - t17830 - t17833 + t17836;
    (t19277,)
}
