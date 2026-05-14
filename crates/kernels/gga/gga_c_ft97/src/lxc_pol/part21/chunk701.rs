//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 701/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk701<F: Float>(t16919: F, t526: F, t27: F, t89: F, t375: F, t4715: F, t4669: F, t12918: F, t16706: F, t16710: F, t16714: F, t16717: F, t16721: F, t16724: F, t16727: F, t16730: F, t16734: F, t16739: F, t16742: F, t16745: F, t16748: F, t16751: F, t16756: F, t16760: F) -> (F, F, F, F) {
    let t16920 = t526 * t16919;
    let t16922 = t89 * t27 * t16920;
    let t16925 = t89 * t375 * t4715;
    let t16928 = t89 * t375 * t4669;
    let t16930 = -t12918 - t16706 / 27.0 + 2.0 / 9.0 * t16710 - t16714 / 9.0 - t16717 / 3.0 + t16721 / 27.0 + 2.0 / 9.0 * t16724 - 5.0 / 81.0 * t16727 - 4.0 / 27.0 * t16730 + t16734 / 9.0 - t16739 + 2.0 / 3.0 * t16742 + t16745 / 54.0 - t16748 / 27.0 + t16751 / 81.0 + t16756 / 3.0 - t16760 / 18.0 - t16922 / 6.0 + t16925 / 18.0 - t16928 / 9.0;
    (t16922, t16925, t16928, t16930)
}
