//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 831/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk831<F: Float>(t2526: F, t581: F, t549: F, t1466: F, t1318: F, t1401: F, t593: F, t571: F, t2442: F, t518: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6188 = t581 * t2526;
    let t6189 = t6188 * t549;
    let t6190 = t1466 * t6189;
    let t6192 = 4.0 / 15.0 * t1318 * t6190;
    let t6193 = t1401 * t2526;
    let t6194 = t6193 * t593;
    let t6195 = t1466 * t6194;
    let t6197 = 4.0 / 15.0 * t571 * t6195;
    let t6198 = t2442 * t518;
    (t6188, t6189, t6190, t6192, t6193, t6194, t6195, t6197, t6198)
}
