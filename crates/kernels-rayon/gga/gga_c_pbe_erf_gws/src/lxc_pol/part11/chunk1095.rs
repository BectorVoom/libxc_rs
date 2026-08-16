//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1095/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1095(t47556: f64, t7062: f64, t7069: f64, t23109: f64, t30666: f64, t40321: f64, t22996: f64, t47545: f64, t47546: f64, t47547: f64, t47548: f64, t47552: f64, t47554: f64, t47555: f64) -> (f64, f64, f64, f64, f64) {
    let t47559 = 16.0_f64 / 9.0_f64 * t7062 * t7069 * t47556;
    let t47560 = 128.0_f64 / 405.0_f64 * t23109;
    let t47561 = 16.0_f64 / 45.0_f64 * t30666;
    let t47562 = 32.0_f64 / 27.0_f64 * t40321;
    let t47563 = 0.14e-19_f64 * t22996 - t47545 + t47546 + t47547 - t47548 + t47552 + t47554 - t47555 - t47559 + t47560 + t47561 + t47562;
    (t47559, t47560, t47561, t47562, t47563)
}
