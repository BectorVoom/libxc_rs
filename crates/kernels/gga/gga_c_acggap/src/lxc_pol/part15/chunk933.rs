//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 933/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk933<F: Float>(t30028: F, t315: F, t2130: F, t3874: F, t615: F, t7930: F, t7911: F, t862: F, t13483: F, t614: F, t851: F, t7941: F) -> (F, F, F, F, F, F, F, F) {
    let t32092 = t315 * t30028;
    let t32123 = t2130 * t3874;
    let t32124 = t615 * t32123;
    let t32130 = t315 * t7930;
    let t32142 = t862 * t7911;
    let t32146 = t614 * t13483 * t2130;
    let t32165 = t851 * t2130;
    let t32181 = t862 * t7941;
    (t32092, t32123, t32124, t32130, t32142, t32146, t32165, t32181)
}
