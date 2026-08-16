//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 878/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk878<F: Float>(t16708: F, t2520: F, t1333: F, t4786: F, t7557: F, t10188: F, t13699: F, t13701: F, t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t7524: F) -> (F, F, F, F) {
    let t16709 = t16708 * t2520;
    let t16715 = t4786 * t1333;
    let t16716 = t7557 * t16715;
    let t16729 = -t7524 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10188 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13699 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13701 + t13703 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t16630 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16634 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16638 - F::cast_from(2.0_f64) * t16642 + F::cast_from(2.0_f64) * t16646 - t16650 / F::cast_from(3.0_f64);
    (t16709, t16715, t16716, t16729)
}
