//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 616/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk616<F: Float>(t1294: F, t565: F, t1289: F, t2104: F, t1524: F, t595: F, t1382: F, t514: F, t211: F, t590: F, t933: F, t1378: F, t331: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3570 = t565 * t1294;
    let t3571 = F::new(8.0) / F::new(15.0) * t3570;
    let t3573 = F::new(4.0) / F::new(5.0) * t2104 * t1289;
    let t3575 = F::new(4.0) / F::new(5.0) * t1524 * t595;
    let t3576 = t514 * t1382;
    let t3577 = t211 * t3576;
    let t3578 = F::new(4.0) / F::new(15.0) * t3577;
    let t3579 = t933 * t590;
    let t3581 = t331 * t1378;
    (t3570, t3571, t3573, t3575, t3576, t3577, t3578, t3579, t3581)
}
