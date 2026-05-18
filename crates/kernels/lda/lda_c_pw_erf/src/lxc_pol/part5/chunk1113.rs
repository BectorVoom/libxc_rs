//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1113/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1113<F: Float>(t2329: F, t739: F, t494: F, t3965: F, t5147: F, t542: F, t4488: F, t4501: F, t348: F) -> (F, F, F, F, F) {
    let t20728 = t2329 * t739;
    let t20729 = t20728 * t494;
    let t20732 = F::new(8.0) / F::new(9.0) * t3965 * t5147 * t20729;
    let t20733 = t20728 * t542;
    let t20736 = F::new(4.0) / F::new(9.0) * t4488 * t4501 * t20733;
    let t20737 = t20728 * t348;
    (t20729, t20732, t20733, t20736, t20737)
}
