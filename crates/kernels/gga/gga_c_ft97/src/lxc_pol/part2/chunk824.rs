//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 824/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk824<F: Float>(t14704: F, t193: F, t89: F, t10400: F, t10279: F, t1186: F, t9733: F, t13730: F, t4044: F, t1471: F, t4092: F, t1701: F, t213: F, t1109: F, t811: F, t820: F) -> (F, F, F, F, F, F, F, F) {
    let t14706 = t89 * t193 * t14704;
    let t14708 = 4.0 / 27.0 * t10400;
    let t14711 = 4.0 / 81.0 * t10279;
    let t14715 = t89 * t9733 * t1186;
    let t14718 = t89 * t13730 * t4044;
    let t14721 = t4092 * t1471;
    let t14722 = t1701 * t213;
    let t14723 = t1109 * t811;
    let t14724 = t14723 * t820;
    (t14706, t14708, t14711, t14715, t14718, t14721, t14722, t14724)
}
