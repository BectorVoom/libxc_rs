//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 407/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk407<F: Float>(t155: F, t4186: F, t1041: F, t971: F, t416: F, t171: F, t4157: F, t4052: F, t4160: F, t1105: F, t362: F, t135: F, t1091: F, t376: F, t1108: F, t150: F) -> (F, F, F, F, F, F, F, F) {
    let t4187 = t155 * t4186;
    let t4189 = t971 * t1041;
    let t4190 = t4189 * t416;
    let t4202 = t171 * t4157;
    let t4203 = t4052 * t4160;
    let t4207 = 1.0 / t1105 / t362;
    let t4208 = t135 * t4207;
    let t4209 = t1091 * t376;
    let t4211 = 1.0 / t1108 / t150;
    (t4187, t4189, t4190, t4202, t4203, t4208, t4209, t4211)
}
