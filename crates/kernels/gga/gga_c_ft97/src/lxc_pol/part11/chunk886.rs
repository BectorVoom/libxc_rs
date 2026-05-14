//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 886/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk886<F: Float>(t39723: F, t39708: F, t39711: F, t39715: F, t39717: F, t39721: F, t39728: F, t39732: F, t39737: F, t39741: F, t39744: F, t39747: F, t39753: F, t39757: F, t40273: F, t39761: F, t39767: F, t39772: F, t39776: F, t39781: F, t39784: F, t39788: F, t39792: F, t39796: F, t40265: F, t40270: F, t40288: F, t40292: F) -> (F, F) {
    let t40546 = 8.0 / 27.0 * t39723;
    let t40555 = -2.0 * t39708 + 4.0 / 3.0 * t39711 - 2.0 / 3.0 * t39715 - 2.0 / 9.0 * t39717 + 4.0 / 3.0 * t39721 + t40546 + 20.0 / 81.0 * t39728 + t39732 / 3.0 + 2.0 / 9.0 * t39737 + 2.0 / 9.0 * t39741 + 4.0 / 9.0 * t39744 - 4.0 / 27.0 * t39747 + 4.0 / 3.0 * t39753 - 2.0 / 3.0 * t39757;
    let t40567 = 56.0 / 81.0 * t40273;
    let t40570 = 2.0 / 9.0 * t39761 + 4.0 / 3.0 * t39767 + t39772 - 4.0 / 3.0 * t39776 - 40.0 / 243.0 * t39781 + 4.0 / 9.0 * t39784 + 4.0 / 3.0 * t39788 + t39792 / 3.0 - t39796 / 9.0 - t40265 / 6.0 + 4.0 * t40270 + t40567 - 6.0 * t40288 - t40292 / 18.0;
    (t40555, t40570)
}
