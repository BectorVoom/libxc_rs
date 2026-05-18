//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 423/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk423<F: Float>(t1004: F, t446: F, t1131: F, t388: F, t155: F, t1041: F, t971: F, t416: F, t171: F, t4157: F, t4052: F, t4160: F) -> (F, F, F, F, F, F) {
    let t4183 = t1004 * t446;
    let t4186 = t388 * t1131;
    let t4187 = t155 * t4186;
    let t4189 = t971 * t1041;
    let t4190 = t4189 * t416;
    let t4202 = t171 * t4157;
    let t4203 = t4052 * t4160;
    (t4183, t4187, t4189, t4190, t4202, t4203)
}
