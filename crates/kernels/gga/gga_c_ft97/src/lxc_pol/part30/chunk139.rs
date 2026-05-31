//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 139/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk139<F: Float>(t319: F, t824: F, t840: F, t305: F, t303: F, t458: F, t295: F, t665: F) -> (F, F, F, F) {
    let t842 = t840 * t319 * t824;
    let t845 = F::cast_from(1.0_f64) / t305;
    let t847 = t458 * t303 / F::cast_from(3.0_f64);
    let t848 = t665 * t295;
    (t842, t845, t847, t848)
}
