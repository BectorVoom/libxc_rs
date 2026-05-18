//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 511/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk511<F: Float>(t2549: F, t522: F, t519: F, t1460: F, t2325: F, t1459: F, t2166: F, t806: F, t1440: F) -> (F, F, F, F, F, F, F) {
    let t2550 = t522 * t2549;
    let t2552 = F::new(4.0) / F::new(45.0) * t519 * t2550;
    let t2553 = t1460 * t2325;
    let t2554 = t1459 * t2553;
    let t2556 = F::new(4.0) / F::new(27.0) * t519 * t2554;
    let t2557 = t2166 * t806;
    let t2558 = t1440 * t2557;
    (t2550, t2552, t2553, t2554, t2556, t2557, t2558)
}
