//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 996/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk996(t19: f64, t5697: f64, t713: f64, t799: f64, t17297: f64, t17300: f64, t17302: f64, t17305: f64, t17308: f64, t17310: f64, t17312: f64, t17316: f64, t17318: f64, t17326: f64) -> f64 {
    let t18237 = 0.27631489407716049382e-3_f64 * t5697 * t19 * t799 * t713;
    let t18238 = -t17297 + t17300 + t17302 + t18237 + t17305 + t17308 + t17310 + t17312 + t17316 + t17318 + t17326;
    t18238
}
