//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 705/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk705<F: Float>(t2409: F, t3067: F, t4097: F, t4034: F, t4046: F, t4024: F, t4030: F, t4036: F, t4040: F, t4044: F, t4050: F) -> (F, F, F, F) {
    let t4099 = t2409 * t3067 * t4097;
    let t4104 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t4034;
    let t4108 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t4046;
    let t4110 = t4024 / F::cast_from(48.0_f64) - t4030 / F::cast_from(48.0_f64) - t4104 - t4036 / F::cast_from(24.0_f64) + t4040 / F::cast_from(384.0_f64) - t4044 / F::cast_from(384.0_f64) - t4108 - t4050 / F::cast_from(192.0_f64);
    (t4099, t4104, t4108, t4110)
}
