//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 608/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk608<F: Float>(t3565: F, t581: F, t3564: F, t190: F, t3431: F, t681: F, t1351: F, t680: F) -> (F, F, F, F, F) {
    let t3566 = t3565 * t581;
    let t3568 = F::cast_from(12.0_f64) * t3564 * t3566;
    let t3569 = t190 * t3431;
    let t3571 = F::cast_from(4.0_f64) * t681 * t3569;
    let t3572 = t680 * t1351;
    (t3566, t3568, t3569, t3571, t3572)
}
