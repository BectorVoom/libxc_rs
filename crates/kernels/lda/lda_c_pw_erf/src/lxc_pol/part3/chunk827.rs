//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 827/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk827<F: Float>(t1234: F, t1513: F, t184: F, t202: F, t4024: F, t3465: F, t493: F, t514: F, t3384: F, t511: F, t1298: F, t3387: F, t1386: F, t3455: F, t1472: F, t3763: F) -> (F, F, F, F, F, F, F) {
    let t9602 = t1513 * t1234;
    let t9615 = t202 * t4024 * t184;
    let t9619 = t493 * t514 * t3465;
    let t9621 = t511 * t3384;
    let t9627 = t1298 * t3387;
    let t9629 = t3455 * t1386;
    let t9645 = t1472 * t3763;
    (t9602, t9615, t9619, t9621, t9627, t9629, t9645)
}
