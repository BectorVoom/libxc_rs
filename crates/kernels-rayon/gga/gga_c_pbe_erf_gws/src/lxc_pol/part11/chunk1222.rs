//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1222/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1222(t36699: f64, t18424: f64, t18428: f64, t18432: f64, t18435: f64, t18439: f64, t18445: f64, t18452: f64, t18456: f64, t18460: f64, t18467: f64, t18471: f64, t18474: f64, t48935: f64) -> (f64, f64) {
    let t49415 = 35.0_f64 / 72.0_f64 * t36699;
    let t49416 = t18424 - t18428 + t18432 - t18435 + t18439 - t18445 - t18452 + t18456 - t18460 + t18467 - t18471 - t18474 - t48935;
    (t49415, t49416)
}
