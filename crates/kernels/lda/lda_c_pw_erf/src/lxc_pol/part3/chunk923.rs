//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 923/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk923<F: Float>(t1987: F, t3709: F, t1446: F, t4856: F, t12138: F, t12141: F, t12145: F, t12146: F, t12148: F, t12150: F, t12152: F, t12156: F, t12163: F, t12165: F, t12169: F, t4859: F) -> (F, F, F, F) {
    let t12171 = 8.0 / 15.0 * t3709 * t1987;
    let t12173 = 8.0 / 15.0 * t1446 * t4856;
    let t12174 = -t12138 + t12141 - t12145 - t12146 + t12148 + t12150 - t12152 - t12156 - t12163 - t12165 - t12169 - t12171 - t12173;
    let t12176 = 8.0 / 3.0 * t1446 * t4859;
    (t12171, t12173, t12174, t12176)
}
