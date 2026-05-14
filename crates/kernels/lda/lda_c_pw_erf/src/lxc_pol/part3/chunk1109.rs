//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1109/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1109<F: Float>(t13114: F, t13120: F, t13127: F, t13129: F, t13131: F, t13133: F, t13135: F, t13137: F, t13139: F, t13141: F, t13143: F, t13145: F, t13147: F, t13152: F, t13154: F, t13162: F, t13164: F, t13166: F, t13171: F, t13175: F, t13177: F, t13179: F, t13182: F, t13187: F, t13189: F, t13191: F) -> (F, F) {
    let t15071 = t13114 - t13120 - t13127 - t13129 - t13131 + t13133 - t13135 - t13137 + t13139 + t13141 + t13143 + t13145 + t13147;
    let t15072 = t13152 + t13154 - t13162 + t13164 - t13166 + t13171 + t13175 - t13177 + t13179 + t13182 - t13187 - t13189 + t13191;
    (t15071, t15072)
}
