//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 990/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk990<F: Float>(t2283: F, t7944: F, t40965: F, t8620: F, t22: F, t235: F, t34812: F, t40978: F, t16503: F, t35039: F, t571: F, t7461: F) -> (F, F, F, F) {
    let t41730 = t7944 * t2283;
    let t41735 = t8620 * t40965;
    let t41738 = t235 * t34812 * t22;
    let t41739 = t41738 * t40978;
    let t41745 = t16503 * t35039 * t571 * t7461;
    (t41730, t41735, t41739, t41745)
}
