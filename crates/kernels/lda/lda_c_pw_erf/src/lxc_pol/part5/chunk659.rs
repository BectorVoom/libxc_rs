//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 659/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk659<F: Float>(t1814: F, t925: F, t474: F, t763: F, t426: F, t1856: F, t431: F, t325: F, t1686: F, t767: F, t933: F, t1833: F, t415: F) -> (F, F, F, F, F, F, F, F) {
    let t5504 = t1814 * t925;
    let t5506 = t474 * t763;
    let t5507 = t426 * t5506;
    let t5509 = t431 * t1856;
    let t5511 = F::new(1.46904) * t5509 * t325;
    let t5512 = t1686 * t767;
    let t5513 = t5512 * t933;
    let t5515 = t415 * t1833;
    (t5504, t5506, t5507, t5509, t5511, t5512, t5513, t5515)
}
