//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1133/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1133<F: Float>(t14423: F, t361: F, t3223: F, t13917: F, t13911: F, t13925: F, t13930: F, t14397: F, t14400: F, t14404: F, t14416: F, t14420: F, t2498: F, t3040: F, t4002: F, t6793: F, t827: F, t8629: F, t8654: F, t8793: F) -> (F, F) {
    let t14424 = t361 * t14423;
    let t14425 = t14424 * t3223;
    let t14426 = t13917 * t14425;
    let t14432 = -t827 * t14397 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14400 + t6793 * t14404 / F::cast_from(48.0_f64) - t8654 * t4002 / F::cast_from(96.0_f64) + t8629 * t13925 / F::cast_from(96.0_f64) + t8793 * t13930 / F::cast_from(48.0_f64) + t8793 * t13911 / F::cast_from(48.0_f64) - t14416 / F::cast_from(1536.0_f64) + t6793 * t14420 / F::cast_from(48.0_f64) - t14426 / F::cast_from(1536.0_f64) - t3040 * t4002 / F::cast_from(96.0_f64) - t2498 * t4002 / F::cast_from(96.0_f64);
    (t14425, t14432)
}
