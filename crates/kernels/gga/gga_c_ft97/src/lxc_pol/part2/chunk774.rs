//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 774/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk774<F: Float>(t13722: F, t13700: F, t13704: F, t13708: F, t13719: F, t9520: F, t9701: F, t9723: F, t9727: F, t9730: F, t9735: F, t2371: F, t3821: F, t713: F, t193: F, t89: F) -> (F, F) {
    let t13723 = 4.0 / 81.0 * t13722;
    let t13724 = t13700 / 6.0 - 4.0 / 9.0 * t13704 + 4.0 / 27.0 * t13708 + t9723 / 27.0 + 2.0 / 81.0 * t9727 - 8.0 / 81.0 * t9735 - 8.0 / 27.0 * t9701 - 2.0 / 9.0 * t9730 + t9520 / 9.0 - 2.0 * t13719 - t13723;
    let t13725 = t2371 * t3821;
    let t13726 = t13725 * t713;
    let t13728 = t89 * t193 * t13726;
    (t13724, t13728)
}
