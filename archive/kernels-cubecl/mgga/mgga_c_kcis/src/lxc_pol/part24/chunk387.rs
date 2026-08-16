//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 387/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk387<F: Float>(t138: F, t2421: F, t86: F, t66: F, t747: F, t119: F, t85: F) -> (F, F, F, F) {
    let t2423 = t86 * t2421 * t138;
    let t2425 = t66 * t747;
    let t2427 = t86 * t2425 * t138;
    let t2429 = t85 * t119;
    (t2423, t2425, t2427, t2429)
}
