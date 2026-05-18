//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 930/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk930<F: Float>(t1155: F, t603: F, t230: F, t4222: F, t331: F, t3615: F, t10042: F, t3606: F, t2061: F, t590: F, t1375: F, t933: F) -> (F, F, F, F, F, F, F) {
    let t10172 = F::new(0.004413481481481482) * t1155 * t603;
    let t10173 = t4222 * t230;
    let t10178 = t331 * t3615;
    let t10195 = F::new(0.3732469135802469) * t10042;
    let t10196 = t331 * t3606;
    let t10202 = t2061 * t590;
    let t10204 = t933 * t1375;
    (t10172, t10173, t10178, t10195, t10196, t10202, t10204)
}
