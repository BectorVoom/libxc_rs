//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 581/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk581<F: Float>(t143: F, t2610: F, t1611: F, t1623: F, t1927: F, t2387: F, t2391: F, t2395: F, t2399: F, t2404: F, t2409: F, t2427: F, t2445: F, t225: F, t2363: F) -> (F, F, F) {
    let t2647 = t143 * t2610;
    let t2657 = t1611 + t2387 - t2391 + t2395 - t2399 + t1623 + 0.21642082724729686 * t1927 + t2404 + t2409 + t2427 + t2445;
    let t2660 = t2363 * t225;
    (t2647, t2657, t2660)
}
