//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 540/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk540<F: Float>(t2136: F, t7494: F, t649: F, t833: F, t27: F, t2134: F, t504: F, t880: F) -> (F, F, F, F) {
    let t7495 = t7494 * t2136;
    let t7497 = t649 * t833;
    let t7498 = t27 * t7497;
    let t7499 = t2134 * t7498;
    let t7501 = t504 * t880;
    (t7495, t7498, t7499, t7501)
}
