//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 596/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk596<F: Float>(t3750: F, t801: F, t274: F, t1095: F, t688: F, t231: F, t1193: F, t278: F, t2014: F, t2394: F, t2710: F, t4068: F, t4069: F, t807: F) -> (F, F, F, F) {
    let t4072 = t801 * t3750;
    let t4073 = t4072 * t274;
    let t4075 = t1095 * t688;
    let t4077 = t231 * t4075 * t274;
    let t4080 = t1193 * t688;
    let t4083 = t3750 * t278;
    let t4088 = -F::new(0.11705142615505742e0) * t4068 * t4069 + F::new(0.23410285231011484e0) * t4073 - F::new(0.26564305359272358183e-2) * t2014 * t4077 + F::new(0.319782988780431561e-1) * t2710 * t4080 - F::new(0.532971647967385935e-1) * t807 * t4083 + F::new(0.13977476158628290272e-1) * t2394 * t4080;
    (t4072, t4077, t4083, t4088)
}
