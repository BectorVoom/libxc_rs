//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 694/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk694<F: Float>(t525: F, t6198: F, t2158: F, t4763: F, t2146: F, t2163: F, t2424: F, t518: F) -> (F, F, F, F) {
    let t6200 = F::new(4.0) / F::new(45.0) * t6198 * t525;
    let t6202 = F::new(8.0) / F::new(15.0) * t4763 * t2158;
    let t6204 = F::new(8.0) / F::new(15.0) * t2146 * t2163;
    let t6205 = t2424 * t518;
    (t6200, t6202, t6204, t6205)
}
