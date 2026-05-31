//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 990/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk990<F: Float>(t1424: F, t6945: F, t2568: F, t3972: F, t7553: F, t6930: F, t97299: F, t1168: F, t140686: F, t10157: F, t1403: F, t140649: F, t1454: F, t193: F, t2354: F, t27956: F, t27963: F, t27976: F, t33243: F, t33253: F, t33535: F, t35267: F, t35270: F, t3821: F, t3837: F, t5996: F, t6002: F, t6008: F, t684: F, t7437: F) -> (F, F, F, F) {
    let t149854 = t1424 * t6945;
    let t149865 = t2568 * t7553 * t3972;
    let t149867 = t97299 * t6930;
    let t149870 = t140686 * t1168;
    let t149880 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1403 * t193 * t6008 * t1454 * t3821 - t1403 * t193 * t33253 * t27963 / F::cast_from(3.0_f64) + t140649 / F::cast_from(9.0_f64) - t6002 * t2354 * t149854 * t684 / F::cast_from(9.0_f64) + t6002 * t10157 * t33535 * t3837 - t7437 * t27976 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) * t149865 + F::cast_from(8.0_f64) * t149867 + t5996 * t35267 - F::cast_from(2.0_f64) * t149870 + t1403 * t193 * t33243 * t27956 + t1403 * t193 * t33243 * t27963 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5996 * t35270;
    (t149865, t149867, t149870, t149880)
}
