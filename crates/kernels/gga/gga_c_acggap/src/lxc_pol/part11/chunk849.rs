//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 849/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk849<F: Float>(t29948: F, t301: F, t694: F, t1268: F, t1679: F, t2541: F, t8022: F, t96: F, t1674: F, t7278: F, t922: F, t811: F, t9097: F) -> (F, F, F, F, F) {
    let t29950 = t694 * t29948 * t301;
    let t29953 = t1679 * t2541 * t1268;
    let t29955 = t96 * t8022;
    let t29958 = t1674 * t7278 * t922;
    let t29961 = t1679 * t9097 * t811;
    (t29950, t29953, t29955, t29958, t29961)
}
