//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 972/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk972<F: Float>(t2124: F, t848: F, t7911: F, t862: F, t7898: F, t13483: F, t2130: F, t614: F, t2132: F, t3037: F, t609: F, t2122: F, t2138: F, t879: F) -> (F, F, F, F) {
    let t32135 = t848 * t2124;
    let t32142 = t862 * t7911;
    let t32143 = t32142 * t7898;
    let t32146 = t614 * t13483 * t2130;
    let t32150 = F::new(0.10408353825846239354e2) * t32146 * t2132 * t609 * t3037;
    let t32157 = t2138 * t2132 * t2122 * t879;
    (t32135, t32143, t32150, t32157)
}
