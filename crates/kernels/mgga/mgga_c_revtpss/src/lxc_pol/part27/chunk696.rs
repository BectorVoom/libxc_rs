//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 696/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk696<F: Float>(t2247: F, t7565: F, t55: F, t60: F, t606: F, t6971: F, t72: F, t1927: F) -> (F, F, F, F, F) {
    let t7566 = t2247 * t7565;
    let t7571 = t55 * t60;
    let t7574 = -5.0 / 6.0 * t7571 * t606 + t6971;
    let t7575 = t7574 * t72;
    let t7576 = t7575 * t1927;
    (t7566, t7571, t7574, t7575, t7576)
}
