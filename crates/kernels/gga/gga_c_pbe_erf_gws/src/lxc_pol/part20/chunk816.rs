//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 816/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk816<F: Float>(t5219: F, t572: F, t108: F, t182: F, t267: F, t1764: F, t1660: F, t597: F, t1663: F, t2647: F, t723: F, t2650: F) -> (F, F, F, F, F, F) {
    let t7055 = t5219 * t572;
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7074 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2647 * t723;
    let t7075 = t2650 * t723;
    (t7055, t7062, t7063, t7069, t7074, t7075)
}
