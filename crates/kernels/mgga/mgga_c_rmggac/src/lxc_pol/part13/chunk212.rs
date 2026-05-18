//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 212/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk212<F: Float>(t27: F, t29: F, t833: F, t32: F, t830: F) -> (F, F, F) {
    let t843 = t833 * t29 * t27;
    let t846 = t27 * t32 * t830;
    let t847 = F::new(0.12222222222222222222e0) * t846;
    let t848 = F::new(5.0) / F::new(18.0) * t843 + t847;
    (t846, t847, t848)
}
