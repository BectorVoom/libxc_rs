//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 889/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk889<F: Float>(t39708: F, t39711: F, t39715: F, t39717: F, t39721: F, t39723: F, t39728: F, t39732: F, t39737: F, t39741: F, t39744: F, t39747: F, t39753: F, t39757: F, t39761: F, t39767: F, t39772: F, t39776: F, t39781: F, t39784: F, t39788: F, t39792: F, t39796: F, t40265: F, t40270: F, t40273: F, t40283: F, t40288: F, t40292: F, t40297: F) -> (F, F) {
    let t40627 = -4.0 * t39708 + 8.0 / 3.0 * t39711 - 4.0 / 3.0 * t39715 - 4.0 / 9.0 * t39717 + 8.0 / 3.0 * t39721 + 16.0 / 27.0 * t39723 + 40.0 / 81.0 * t39728 + 2.0 / 3.0 * t39732 + 4.0 / 9.0 * t39737 + 4.0 / 9.0 * t39741 + 8.0 / 9.0 * t39744 - 8.0 / 27.0 * t39747 + 8.0 / 3.0 * t39753 - 4.0 / 3.0 * t39757 + 4.0 / 9.0 * t39761;
    let t40644 = 8.0 / 3.0 * t39767 + 2.0 * t39772 - 8.0 / 3.0 * t39776 - 80.0 / 243.0 * t39781 + 8.0 / 9.0 * t39784 + 8.0 / 3.0 * t39788 + 2.0 / 3.0 * t39792 - 2.0 / 9.0 * t39796 - t40265 / 3.0 + 8.0 * t40270 + 112.0 / 81.0 * t40273 - 5.0 / 16.0 * t40283 - 12.0 * t40288 - t40292 / 9.0 + 40.0 / 27.0 * t40297;
    (t40627, t40644)
}
