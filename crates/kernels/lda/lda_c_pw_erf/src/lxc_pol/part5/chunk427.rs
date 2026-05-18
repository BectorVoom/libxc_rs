//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 427/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk427<F: Float>(t1313: F, t1996: F, t519: F, t558: F, t811: F, t352: F) -> (F, F, F, F) {
    let t1997 = t1313 * t1996;
    let t1999 = F::new(4.0) / F::new(45.0) * t519 * t1997;
    let t2000 = t811 * t558;
    let t2001 = t2000 * t352;
    (t1997, t1999, t2000, t2001)
}
