//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 723/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk723<F: Float>(t363: F, t6743: F, t1014: F, t1018: F, t1012: F, sigma0: F) -> (F, F, F, F) {
    let t6744 = t6743 * t363;
    let t6753 = t1014 * sigma0;
    let t6754 = t6753 * t1018;
    let t6755 = t1012 * t6754;
    (t6744, t6753, t6754, t6755)
}
