//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1157/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1157<F: Float>(t21240: F, t519: F, t5250: F, t21204: F, t21206: F, t21210: F, t21214: F, t21216: F, t21218: F, t21222: F, t21224: F, t21228: F, t21230: F, t21234: F, t21238: F) -> (F, F) {
    let t21243 = F::new(128.0) / F::new(27.0) * t519 * t5250 * t21240;
    let t21244 = t21204 + t21206 + t21210 - t21214 + t21216 + t21218 - t21222 - t21224 - t21228 - t21230 - t21234 + t21238 - t21243;
    (t21243, t21244)
}
