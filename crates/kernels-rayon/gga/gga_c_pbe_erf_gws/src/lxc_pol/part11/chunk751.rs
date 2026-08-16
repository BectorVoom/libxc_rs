//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 751/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk751(t50: f64, t12350: f64, t12355: f64, t2465: f64, t3354: f64, t4767: f64, t52: f64, t12349: f64, t59: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t12359 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t4767 * t12350 + 4.0_f64 / 3.0_f64 * t2465 * t3354 + 4.0_f64 / 3.0_f64 * t52 * t12355);
    let t12361 = (t12349 + t12359) * t59;
    t12361
}
