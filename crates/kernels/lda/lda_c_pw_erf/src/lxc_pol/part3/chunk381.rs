//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 381/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk381<F: Float>(t494: F, t529: F, t542: F, t1440: F, t1325: F, t510: F, t518: F) -> (F, F, F, F) {
    let t1442 = t529 * t494 * t542;
    let t1443 = t1440 * t1442;
    let t1445 = 8.0 / 15.0 * t1325 * t1443;
    let t1446 = t510 * t518;
    (t1442, t1443, t1445, t1446)
}
