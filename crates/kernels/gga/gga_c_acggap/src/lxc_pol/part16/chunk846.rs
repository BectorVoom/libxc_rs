//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 846/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk846<F: Float>(t13483: F, t2130: F, t614: F, t2132: F, t3037: F, t609: F, t2122: F, t2138: F, t879: F, t2131: F, t847: F, t7990: F, t7994: F, t851: F, t7998: F, t7987: F) -> (F, F, F, F, F, F) {
    let t32146 = t614 * t13483 * t2130;
    let t32150 = 0.10408353825846239354e2 * t32146 * t2132 * t609 * t3037;
    let t32157 = t2138 * t2132 * t2122 * t879;
    let t32161 = t2131 * t2132 * t2122 * t847;
    let t32163 = t7990 * t7994;
    let t32165 = t851 * t2130;
    let t32167 = 0.26020884564615598386e1 * t32165 * t7998;
    let t32168 = t7987 * t7998;
    (t32150, t32157, t32161, t32163, t32167, t32168)
}
