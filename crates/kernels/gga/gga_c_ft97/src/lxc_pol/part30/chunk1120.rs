//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1120/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1120<F: Float>(t35915: F, t35916: F, t816: F, t2725: F, t6793: F, t4092: F, t150603: F, t7205: F, t811: F, t10364: F, t1200: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t153071 = t35915 * t35916 * t816;
    let t153074 = t2725 * t6793;
    let t153075 = t4092 * t153074;
    let t153077 = t7205 * t150603 * t811;
    let t153080 = t10364 * t6793;
    let t153081 = t1200 * t153080;
    let t153083 = t7205 * t150603 * t820;
    (t153071, t153074, t153075, t153077, t153080, t153081, t153083)
}
