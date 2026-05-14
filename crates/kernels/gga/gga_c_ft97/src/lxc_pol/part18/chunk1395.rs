//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1395/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1395<F: Float>(t147: F, t104104: F, t107745: F, t100044: F, t100045: F, t104076: F, t13: F, t23399: F, t24173: F) -> (F,) {
    let t148 = 10000000.0 <= t147;
    let t107747 = piecewise3(t148, 0.0, t104104 + t107745);
    let tv4rho3sigma3 = t23399 + t24173 + t100044 + t100045 + t13 * (t104076 + t107747);
    (tv4rho3sigma3,)
}
