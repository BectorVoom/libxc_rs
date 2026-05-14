//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 673/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk673<F: Float>(t237: F, t663: F, t8643: F, t2303: F, t2334: F, t2338: F, t88: F, t2342: F, t2371: F, t672: F, t2355: F, t678: F) -> (F, F, F, F, F) {
    let t8646 = 0.71233333333333333334e-1 * t237 * t8643 * t663;
    let t8649 = 0.53425e-1 * t237 * t2303 * t2334;
    let t8650 = t88 * t2338;
    let t8653 = 0.85917146441092277512e0 * t237 * t8650 * t2342;
    let t8655 = 1.0 / t2371 / t672;
    let t8656 = t2355 * t678;
    (t8646, t8649, t8653, t8655, t8656)
}
