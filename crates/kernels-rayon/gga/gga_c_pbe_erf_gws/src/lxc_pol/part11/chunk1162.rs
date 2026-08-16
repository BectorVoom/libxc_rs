//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1162/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1162(t50: f64, t12355: f64, t1412: f64, t18684: f64, t2465: f64, t3354: f64, t47372: f64, t47377: f64, t47733: f64, t52: f64, t9993: f64, t48458: f64, t59: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t48470 = piecewise3(t51, 0.0_f64, 40.0_f64 / 81.0_f64 * t18684 * t47377 - 16.0_f64 / 9.0_f64 * t9993 * t3354 + 4.0_f64 / 3.0_f64 * t1412 * t47733 + 16.0_f64 / 9.0_f64 * t2465 * t12355 + 4.0_f64 / 3.0_f64 * t52 * t47372);
    let t48472 = (t48458 + t48470) * t59;
    t48472
}
