//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 901/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk901<F: Float>(t19: F, t5697: F, t713: F, t799: F, t17297: F, t17300: F, t17302: F, t17305: F, t17308: F, t17310: F, t17312: F, t17316: F, t17318: F, t17326: F, t5385: F, t720: F) -> (F, F) {
    let t18237 = 0.27631489407716049382e-3 * t5697 * t19 * t799 * t713;
    let t18238 = -t17297 + t17300 + t17302 + t18237 + t17305 + t17308 + t17310 + t17312 + t17316 + t17318 + t17326;
    let t18240 = 32.0 / 81.0 * t720 * t5385;
    (t18238, t18240)
}
