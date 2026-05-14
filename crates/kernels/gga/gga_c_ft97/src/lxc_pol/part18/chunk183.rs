//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 183/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk183<F: Float>(t637: F, t639: F, t643: F, t629: F, t631: F, t634: F, t184: F, t21: F, t19: F, t362: F) -> (F, F, F, F, F) {
    let t645 = t637 * t639 * t643;
    let t648 = t629 + t631 * t634 / 6.0 + t631 * t645 / 2.0;
    let t649 = t648 * t184;
    let t650 = t649 * t21;
    let t920 = -t19 - t362;
    (t645, t648, t649, t650, t920)
}
