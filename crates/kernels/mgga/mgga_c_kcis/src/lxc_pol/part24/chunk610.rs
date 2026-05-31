//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 610/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk610<F: Float>(t278: F, t2910: F, t6533: F, t286: F, t6432: F) -> (F, F, F) {
    let t288 = F::cast_from(0.0_f64) < t278;
    let t6534 = t2910 * t6533;
    let t6535 = t286 * t6534;
    let t6539 = piecewise3::<F>(t288, t6432, -t6432);
    (t6534, t6535, t6539)
}
