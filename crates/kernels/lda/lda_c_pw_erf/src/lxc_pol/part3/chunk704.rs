//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 704/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk704<F: Float>(t159: F, t285: F, t4422: F, t1896: F, t477: F, t440: F, t756: F, t2765: F, t1191: F, t169: F, t301: F, t865: F) -> (F, F, F, F, F) {
    let t4425 = F::cast_from(0.0005811348303577384_f64) * t4422 * t159 * t285;
    let t4427 = t1896 * t477 * t285;
    let t4429 = t756 * t440;
    let t4430 = t2765 * t4429;
    let t4435 = t169 * t1191 * t865 * t301;
    (t4425, t4427, t4429, t4430, t4435)
}
