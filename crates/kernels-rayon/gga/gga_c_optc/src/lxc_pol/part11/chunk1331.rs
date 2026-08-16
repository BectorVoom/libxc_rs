//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1331/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1331(t13632: f64, t16922: f64, t23537: f64, t2722: f64, t277: f64, t39204: f64, t4038: f64, t4044: f64, t49197: f64, t49223: f64, t56676: f64, t56681: f64, t56686: f64, t56693: f64, t57628: f64, t57995: f64, t95: f64, t962: f64) -> f64 {
    let t58004 = 2.0_f64 / 3.0_f64 * t49197 - t56676 + t56681 + 2.0_f64 / 3.0_f64 * t49223 - t23537 - 8.0_f64 / 27.0_f64 * t39204 + t56686 + 8.0_f64 / 3.0_f64 * t13632 * t16922 - t56693 + 0.25844881434903430496e-2_f64 * t95 * t277 * t57995 * t962 + 6.0_f64 * t4038 * t2722 * t4044 * t57628;
    t58004
}
