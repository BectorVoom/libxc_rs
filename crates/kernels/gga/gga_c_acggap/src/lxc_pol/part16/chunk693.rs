//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 693/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk693<F: Float>(t2132: F, t7993: F, t2138: F, t609: F, t847: F, t2131: F, t119: F, t2122: F, t159: F, t3874: F) -> (F, F, F, F, F, F, F) {
    let t7994 = t2132 * t7993;
    let t7996 = 0.8673628188205199462e0 * t2138 * t7994;
    let t7997 = t609 * t847;
    let t7998 = t2132 * t7997;
    let t8000 = 0.8673628188205199462e0 * t2131 * t7998;
    let t8001 = t119 * t2122;
    let t8004 = t3874 * t159;
    (t7994, t7996, t7997, t7998, t8000, t8001, t8004)
}
