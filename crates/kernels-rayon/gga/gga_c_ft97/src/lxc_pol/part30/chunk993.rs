//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 993/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk993(t33598: f64, t51340: f64, t13927: f64, t33601: f64, t1403: f64, t140579: f64, t140594: f64, t1425: f64, t193: f64, t27882: f64, t27997: f64, t28002: f64, t28006: f64, t28042: f64, t28461: f64, t33277: f64, t33494: f64, t33499: f64, t35287: f64, t3683: f64, t3837: f64, t42500: f64, t5996: f64, t6002: f64, t6749: f64, t7558: f64) -> (f64, f64, f64) {
    let t149965 = t51340 * t33598;
    let t149967 = t13927 * t33601;
    let t149992 = -2.0_f64 / 3.0_f64 * t5996 * t35287 - 12.0_f64 * t149965 + 8.0_f64 * t149967 - t3683 * t7558 + t1403 * t193 * t1425 * t28461 / 3.0_f64 + t6002 * t140594 * t28042 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t27882 * t33277 - t140579 * t6749 / 18.0_f64 - t33499 * t28002 / 18.0_f64 - t33499 * t28006 / 18.0_f64 + t33499 * t27997 - 4.0_f64 * t6002 * t42500 * t33494 * t3837;
    (t149965, t149967, t149992)
}
