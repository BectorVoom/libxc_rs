//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 328/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk328<F: Float>(t1210: F, t168: F, t270: F, t635: F, t671: F, t155: F, t266: F) -> (F, F, F) {
    let t1213 = F::new(0.053059442957798957) * t168 * t1210 * t270;
    let t1215 = t168 * t635 * t671;
    let t1217 = t266 * t155;
    (t1213, t1215, t1217)
}
