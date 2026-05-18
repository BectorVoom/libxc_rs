//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1159/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1159<F: Float>(t26494: F, t7642: F, t209: F, t2155: F, t7645: F, t8915: F, t2398: F, t26477: F, t8944: F, t26430: F, t7647: F, t7639: F) -> (F, F, F, F, F) {
    let t92082 = t7642 * t26494;
    let t92086 = t2155 * t209 * t7645 * t8915;
    let t92089 = t8944 * t2398 * t26477;
    let t92091 = t26430 * t7647;
    let t92093 = t26430 * t7639;
    (t92082, t92086, t92089, t92091, t92093)
}
