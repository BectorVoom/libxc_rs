//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 770/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk770<F: Float>(t5049: F, t5051: F, t5053: F, t5055: F, t5057: F, t5059: F, t5061: F, t5067: F, t5071: F, t5131: F, t5133: F, t5135: F, t5140: F, t5145: F, t5150: F, t5154: F, t5159: F) -> (F,) {
    let t5863 = -t5049 + t5051 - t5053 - t5055 - t5057 + t5059 + t5061 + t5067 + t5071 - t5131 - t5133 + t5135 - t5140 - t5145 + t5150 - t5154 - t5159;
    (t5863,)
}
