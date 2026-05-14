//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 805/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk805<F: Float>(t13891: F, t13950: F, t14041: F, t14091: F, t14153: F, t14209: F, t14251: F, t14292: F, t9735: F, t9701: F, t13746: F, t13753: F, t13704: F, t13708: F, t13719: F, t13722: F, t13728: F, t13732: F, t13736: F, t13739: F, t13743: F, t13750: F, t9520: F, t9723: F, t9727: F, t9730: F, t9765: F, t9768: F) -> (F, F) {
    let t14295 = t13891 + t13950 + t14041 + t14091 + t14153 + t14209 + t14251 + t14292;
    let t14317 = 4.0 / 81.0 * t9735;
    let t14318 = 4.0 / 27.0 * t9701;
    let t14327 = 2.0 / 9.0 * t13746;
    let t14329 = t13753 / 9.0;
    let t14332 = -2.0 / 9.0 * t13704 + 2.0 / 27.0 * t13708 + t9723 / 54.0 + t9727 / 81.0 - t14317 - t14318 - t9730 / 9.0 + t9520 / 18.0 - t13719 - 2.0 / 81.0 * t13722 + 2.0 / 3.0 * t13728 - 11.0 / 27.0 * t13732 + t13736 / 9.0 - 2.0 / 27.0 * t13739 + t13743 / 3.0 - t14327 - t13750 / 6.0 + t14329 - t9768 / 27.0 - t9765 / 27.0;
    (t14295, t14332)
}
