//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1128/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1128(t14423: f64, t361: f64, t3223: f64, t13917: f64, t13911: f64, t13925: f64, t13930: f64, t14397: f64, t14400: f64, t14404: f64, t14416: f64, t14420: f64, t2498: f64, t3040: f64, t4002: f64, t6793: f64, t827: f64, t8629: f64, t8654: f64, t8793: f64) -> (f64, f64, f64) {
    let t14424 = t361 * t14423;
    let t14425 = t14424 * t3223;
    let t14426 = t13917 * t14425;
    let t14432 = -t827 * t14397 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t14400 + t6793 * t14404 / 48.0_f64 - t8654 * t4002 / 96.0_f64 + t8629 * t13925 / 96.0_f64 + t8793 * t13930 / 48.0_f64 + t8793 * t13911 / 48.0_f64 - t14416 / 1536.0_f64 + t6793 * t14420 / 48.0_f64 - t14426 / 1536.0_f64 - t3040 * t4002 / 96.0_f64 - t2498 * t4002 / 96.0_f64;
    (t14424, t14425, t14432)
}
