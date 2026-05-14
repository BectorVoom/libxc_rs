//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1290/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1290<F: Float>(t16085: F, t16089: F, t16093: F, t16095: F, t16099: F, t16103: F, t16108: F, t16110: F, t16114: F, t16116: F, t16118: F, t16120: F, t16125: F, t16128: F, t16130: F, t16132: F, t16135: F) -> (F,) {
    let t19129 = t16085 - t16089 - t16093 + t16095 - t16099 + t16103 + t16108 + t16110 + t16114 + t16116 + t16118 - t16120 - t16125 - t16128 + t16130 + t16132 + t16135;
    (t19129,)
}
