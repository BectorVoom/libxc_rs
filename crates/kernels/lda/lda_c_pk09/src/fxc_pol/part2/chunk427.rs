//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 427/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk427<F: Float>(t1180: F, t2451: F, t1153: F, t1164: F, t2448: F, t253: F) -> (F, F) {
    let t2452 = t1180 * t2451;
    let t2455 = t1153 - t1164 + 1.28 * t253 * t2448 - 1.28 * t253 * t2452;
    (t2452, t2455)
}
