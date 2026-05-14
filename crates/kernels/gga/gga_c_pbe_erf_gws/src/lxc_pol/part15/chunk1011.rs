//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1011/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1011<F: Float>(t14498: F, t3249: F, t3299: F, t4039: F, t14025: F, t14481: F, t14483: F, t14485: F, t14487: F, t14489: F, t14491: F, t14493: F, t14495: F, t1154: F, t14079: F, t3172: F, t4028: F) -> (F, F, F) {
    let t14499 = t14498 * t3249;
    let t14502 = t4039 * t3299;
    let t14504 = -t14481 / 384.0 + t14483 / 96.0 - t14485 / 768.0 + t14487 / 192.0 - t14489 / 768.0 + t14491 / 96.0 - t14493 / 384.0 - t14495 / 96.0 + t14499 / 256.0 - 7.0 / 288.0 * t14025 + t14502 / 768.0;
    let t14506 = t14079 * t1154;
    let t14508 = t4028 * t3172;
    (t14504, t14506, t14508)
}
