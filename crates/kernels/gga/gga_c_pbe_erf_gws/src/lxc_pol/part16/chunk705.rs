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
    let t4104 = F::new(7.0) / F::new(144.0) * t4034;
    let t4108 = F::new(7.0) / F::new(1152.0) * t4046;
    let t4110 = t4024 / F::new(48.0) - t4030 / F::new(48.0) - t4104 - t4036 / F::new(24.0) + t4040 / F::new(384.0) - t4044 / F::new(384.0) - t4108 - t4050 / F::new(192.0);
    (t4099, t4104, t4108, t4110)
}
