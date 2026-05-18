//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 567/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk567<F: Float>(t2136: F, t7494: F, t649: F, t833: F, t27: F, t2134: F, t504: F, t880: F) -> (F, F, F, F) {
    let t7495 = t7494 * t2136;
    let t7496 = F::new(0.20455996240684006296e-1) * t7495;
    let t7497 = t649 * t833;
    let t7498 = t27 * t7497;
    let t7499 = t2134 * t7498;
    let t7500 = F::new(0.10227998120342003148e-1) * t7499;
    let t7501 = t504 * t880;
    (t7496, t7498, t7500, t7501)
}
