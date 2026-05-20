//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2274/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2274<F: Float>(t17730: F, t5051: F, t3626: F, t3566: F, t489: F, t17728: F) -> (F, F, F) {
    let t17731 = t5051 * t17730;
    let t17732 = t3626 * t17731;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    (t17731, t17732, t17736)
}
