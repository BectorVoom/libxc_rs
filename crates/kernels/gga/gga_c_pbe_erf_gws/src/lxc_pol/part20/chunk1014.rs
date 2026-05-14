//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1014/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1014<F: Float>(t14423: F, t361: F, t3223: F, t13917: F, t13911: F, t13925: F, t13930: F, t14397: F, t14400: F, t14404: F, t14416: F, t14420: F, t2498: F, t3040: F, t4002: F, t6793: F, t827: F, t8629: F, t8654: F, t8793: F) -> (F, F) {
    let t14424 = t361 * t14423;
    let t14425 = t14424 * t3223;
    let t14426 = t13917 * t14425;
    let t14432 = -t827 * t14397 / 96.0 + 7.0 / 288.0 * t14400 + t6793 * t14404 / 48.0 - t8654 * t4002 / 96.0 + t8629 * t13925 / 96.0 + t8793 * t13930 / 48.0 + t8793 * t13911 / 48.0 - t14416 / 1536.0 + t6793 * t14420 / 48.0 - t14426 / 1536.0 - t3040 * t4002 / 96.0 - t2498 * t4002 / 96.0;
    (t14425, t14432)
}
