//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2056/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2056<F: Float>(t14676: F, t4364: F, t837: F, t2646: F, t4365: F, t136: F, t243: F, t220: F) -> (F, F, F) {
    let t14678 = t4364 * t14676 * t837;
    let t14682 = t4364 * t4365 * t2646;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    (t14678, t14682, t14686)
}
