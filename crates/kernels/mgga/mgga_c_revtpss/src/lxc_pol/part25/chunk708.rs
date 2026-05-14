//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 708/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk708<F: Float>(t343: F, t613: F, t136: F, t1007: F, t1968: F, t1967: F, t800: F) -> (F, F, F, F) {
    let t7105 = t613 * t343;
    let t7106 = t7105 * t136;
    let t7110 = t1968 * t1007 / 288.0;
    let t7111 = t1967 * t800;
    (t7105, t7106, t7110, t7111)
}
