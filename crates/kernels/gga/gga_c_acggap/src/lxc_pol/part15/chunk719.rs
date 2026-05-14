//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 719/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk719<F: Float>(t1165: F, t604: F, t8791: F, t7413: F, t1323: F, t7815: F, t2030: F, t1327: F, t2060: F, t2029: F, t568: F) -> (F, F, F, F, F, F, F) {
    let t8793 = t1165 * t604 * t8791;
    let t8794 = t7413 * t8793;
    let t8800 = t7815 * t1323;
    let t8801 = t2030 * t8800;
    let t8803 = t7815 * t1327;
    let t8804 = t2060 * t8803;
    let t8806 = t568 * t2029;
    (t8793, t8794, t8800, t8801, t8803, t8804, t8806)
}
