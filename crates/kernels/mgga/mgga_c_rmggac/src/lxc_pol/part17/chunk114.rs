//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 114/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk114<F: Float>(t338: F, t352: F, t118: F, t323: F, t335: F) -> (F, F) {
    let t353 = t338 * t352;
    let t354 = t118 * t353;
    let t356 = -F::new(0.59871208509319042821e-1) * t323 + F::new(0.59871208509319042821e-1) * t335 + F::new(0.19957069503106347607e-1) * t354;
    (t354, t356)
}
