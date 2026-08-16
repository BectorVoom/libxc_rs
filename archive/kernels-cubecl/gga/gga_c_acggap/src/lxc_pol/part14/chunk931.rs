//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 931/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk931<F: Float>(t32123: F, t615: F, t315: F, t7930: F, t2124: F, t848: F, t7911: F, t862: F, t7898: F, t13483: F, t2130: F, t614: F) -> (F, F, F, F, F) {
    let t32124 = t615 * t32123;
    let t32130 = t315 * t7930;
    let t32135 = t848 * t2124;
    let t32142 = t862 * t7911;
    let t32143 = t32142 * t7898;
    let t32146 = t614 * t13483 * t2130;
    (t32124, t32130, t32135, t32143, t32146)
}
