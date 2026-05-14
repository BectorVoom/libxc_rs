//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 691/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk691<F: Float>(t309: F, t314: F, t180: F, t1219: F, t2130: F, t615: F) -> (F, F, F, F, F) {
    let t7922 = t309 * t309;
    let t7923 = t7922 * t314;
    let t7924 = t7923 * t180;
    let t7930 = t2130 * t1219;
    let t7931 = t615 * t7930;
    (t7922, t7923, t7924, t7930, t7931)
}
