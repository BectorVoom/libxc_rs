//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 722/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk722<F: Float>(t1208: F, t840: F, t1205: F, t810: F, t2376: F, t2409: F, t1206: F, t892: F, t338: F, t938: F, t3067: F, t4034: F) -> (F, F, F, F, F, F, F) {
    let t4087 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t840 * t1208;
    let t4088 = t1205 * t810;
    let t4090 = t2409 * t2376 * t4088;
    let t4093 = t892 * t1206;
    let t4094 = t338 * t4093;
    let t4097 = t1205 * t938;
    let t4099 = t2409 * t3067 * t4097;
    let t4104 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t4034;
    (t4087, t4088, t4090, t4094, t4097, t4099, t4104)
}
