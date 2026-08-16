//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 791/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk791<F: Float>(t10703: F, t10704: F, t2749: F, t2867: F, t840: F, t2801: F, t875: F, t2843: F, t296: F, t2739: F, t824: F, t2862: F, t319: F) -> (F, F, F, F, F, F, F) {
    let t10705 = t10703 * t10704;
    let t10709 = t840 * t2749 * t2867;
    let t10712 = t875 * t2801;
    let t10713 = t2843 * t10712;
    let t10714 = t296 * t10713;
    let t10717 = t2739 * t824;
    let t10719 = t2862 * t319 * t10717;
    (t10705, t10709, t10712, t10713, t10714, t10717, t10719)
}
