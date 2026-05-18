//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 592/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk592<F: Float>(t376: F, t6460: F, t497: F, t6455: F, t28: F, t6413: F, t92: F, t3266: F, t5502: F, t8411: F, t22917: F, t925: F) -> (F, F, F, F, F) {
    let t25545 = t376 * t6460;
    let t25552 = t6455 * t497;
    let t25553 = t28 * t25552;
    let t25558 = t6413 * t92;
    let t25564 = t8411 * t5502 * t3266;
    let t25569 = t22917 * t925;
    (t25545, t25553, t25558, t25564, t25569)
}
