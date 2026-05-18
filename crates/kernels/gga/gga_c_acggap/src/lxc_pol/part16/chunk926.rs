//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 926/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk926<F: Float>(t7976: F, t880: F, t2155: F, t30005: F, t2122: F, t2132: F, t7885: F, t864: F, t1219: F, t615: F, t7911: F, t862: F, t865: F) -> (F, F, F, F, F) {
    let t31916 = t7976 * t880;
    let t31926 = t30005 * t2155;
    let t31955 = t7885 * t2132 * t2122 * t864;
    let t31965 = t615 * t7911 * t1219;
    let t31969 = t862 * t2122 * t865;
    (t31916, t31926, t31955, t31965, t31969)
}
