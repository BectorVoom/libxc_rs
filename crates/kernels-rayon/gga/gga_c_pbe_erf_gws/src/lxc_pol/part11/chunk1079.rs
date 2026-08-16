//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1079/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1079(t42953: f64, t47343: f64, t47347: f64, t47351: f64, t47355: f64, t47359: f64, t47363: f64, t47364: f64, t47366: f64, t47368: f64, t47369: f64, t16576: f64, t39: f64) -> (f64, f64) {
    let t47370 = t47343 + t47347 + t47351 - t47355 - t47359 - t47363 - t47364 + t47366 - 8.0_f64 / 45.0_f64 * t42953 - t47368 - t47369;
    let t47371 = -t39 - t16576;
    (t47370, t47371)
}
