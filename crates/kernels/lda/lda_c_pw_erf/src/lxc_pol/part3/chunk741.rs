//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 741/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk741<F: Float>(t5512: F, t933: F, t1833: F, t415: F, t325: F, t1652: F, t760: F, t156: F, t1844: F, t426: F, t3234: F, t739: F, t1558: F, t34: F, t3243: F, t743: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5513 = t5512 * t933;
    let t5515 = t415 * t1833;
    let t5517 = 0.9743416666666667 * t5515 * t325;
    let t5518 = t1652 * t760;
    let t5519 = t5518 * t933;
    let t5520 = 0.3247805555555556 * t5519;
    let t5521 = t156 * t1844;
    let t5523 = t426 * t5521 / 3.0;
    let t5524 = t3234 * t739;
    let t5527 = t1558 * t34;
    let t5536 = t3243 * t743;
    (t5513, t5515, t5517, t5518, t5520, t5521, t5523, t5524, t5527, t5536)
}
