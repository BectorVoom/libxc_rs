//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 783/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk783<F: Float>(t21457: F, t2354: F, t446: F, t13722: F, t13739: F, t17720: F, t21433: F, t21437: F, t21440: F, t21444: F, t21448: F, t21451: F, t21455: F, t9699: F) -> (F, F, F) {
    let t21458 = t2354 * t21457;
    let t21459 = t446 * t21458;
    let t21462 = -F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13722 - t9699 - t17720 / F::cast_from(9.0_f64) - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t21433 - t21437 / F::cast_from(3.0_f64) + t21440 / F::cast_from(3.0_f64) + t21444 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t21448 - t21451 / F::cast_from(9.0_f64) + t21455 / F::cast_from(6.0_f64) + t21459 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13739;
    (t21458, t21459, t21462)
}
