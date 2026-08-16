//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 988/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk988<F: Float>(t2128: F, t7927: F, t4083: F, t2119: F, t7931: F, t13632: F, t13618: F, t20292: F, t26138: F, t26150: F, t26159: F, t30288: F, t30292: F, t30296: F, t30300: F, t30303: F, t30306: F) -> (F, F, F, F, F) {
    let t30318 = t7927 * t2128;
    let t30319 = t30318 * t4083;
    let t30326 = t7931 * t2119;
    let t30327 = t13632 * t30326;
    let t30339 = -t13618 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t20292 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t26138 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26150 + t26159 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t30288 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t30292 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t30296 - F::cast_from(2.0_f64) * t30300 + F::cast_from(2.0_f64) * t30303 - t30306 / F::cast_from(3.0_f64);
    (t30318, t30319, t30326, t30327, t30339)
}
