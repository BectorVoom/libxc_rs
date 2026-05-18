//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 553/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk553<F: Float>(t1035: F, t4245: F, t124: F, t56: F, t2029: F, t142: F, t174: F) -> (F, F, F, F) {
    let t4246 = t1035 * t4245;
    let t4254 = t124 * t56;
    let t4255 = t4254 * t2029;
    let t4256 = t142 * t174;
    (t4246, t4254, t4255, t4256)
}
