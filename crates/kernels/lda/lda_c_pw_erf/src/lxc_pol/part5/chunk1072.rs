//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1072/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1072<F: Float>(t15455: F, t11372: F, t11374: F, t11376: F, t20057: F, t20058: F, t20059: F, t20060: F, t20062: F, t20063: F, t20067: F, t20068: F, t20069: F, t8285: F, t8290: F, t8296: F, t8300: F, t8301: F, t8356: F) -> (F, F) {
    let t20070 = F::new(60.0) * t15455;
    let t20071 = -t20057 + t8285 + t20058 + t8290 + t20059 - t8296 - t20060 - t11372 - t11374 + t20062 - t8300 + t20063 + t11376 - F::cast_from(1.825614615114074_f64) * t8301 - t20067 + t20068 - t8356 - t20069 + t20070;
    (t20070, t20071)
}
