//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1032/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1032<F: Float>(t13984: F, t14657: F, t13875: F, t13884: F, t13886: F, t13895: F, t14624: F, t14629: F, t14634: F, t14640: F, t14643: F, t14649: F, t14652: F, t14655: F, t2408: F, t3066: F, t335: F) -> (F,) {
    let t14658 = t14657 * t13984;
    let t14660 = -7.0 / 144.0 * t13875 + t3066 * t14624 / 48.0 + t2408 * t14629 / 48.0 + t14634 / 768.0 + 5.0 / 768.0 * t14640 - t335 * t14643 / 96.0 + 7.0 / 288.0 * t13884 + 7.0 / 288.0 * t13886 - t14649 / 96.0 - t2408 * t14652 / 24.0 + t13895 + 7.0 / 1152.0 * t14655 - t14658 / 96.0;
    (t14660,)
}
