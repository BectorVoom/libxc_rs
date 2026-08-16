//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 990/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk990(t1424: f64, t6945: f64, t2568: f64, t3972: f64, t7553: f64, t6930: f64, t97299: f64, t1168: f64, t140686: f64, t10157: f64, t1403: f64, t140649: f64, t1454: f64, t193: f64, t2354: f64, t27956: f64, t27963: f64, t27976: f64, t33243: f64, t33253: f64, t33535: f64, t35267: f64, t35270: f64, t3821: f64, t3837: f64, t5996: f64, t6002: f64, t6008: f64, t684: f64, t7437: f64) -> (f64, f64, f64, f64) {
    let t149854 = t1424 * t6945;
    let t149865 = t2568 * t7553 * t3972;
    let t149867 = t97299 * t6930;
    let t149870 = t140686 * t1168;
    let t149880 = -2.0_f64 / 3.0_f64 * t1403 * t193 * t6008 * t1454 * t3821 - t1403 * t193 * t33253 * t27963 / 3.0_f64 + t140649 / 9.0_f64 - t6002 * t2354 * t149854 * t684 / 9.0_f64 + t6002 * t10157 * t33535 * t3837 - t7437 * t27976 / 3.0_f64 + 4.0_f64 * t149865 + 8.0_f64 * t149867 + t5996 * t35267 - 2.0_f64 * t149870 + t1403 * t193 * t33243 * t27956 + t1403 * t193 * t33243 * t27963 - 2.0_f64 / 3.0_f64 * t5996 * t35270;
    (t149865, t149867, t149870, t149880)
}
