//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 846/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk846<F: Float>(t3000: F, t364: F, t89: F, t1572: F, t7773: F, t13: F, t7741: F, t18: F, t7742: F) -> (F, F, F, F, F) {
    let t37382 = t89 * t3000 * t364;
    let t37383 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t37382;
    let t37385 = t89 * t7773 * t1572;
    let t37386 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t37385;
    let t37387 = t7741 * t13;
    let t37388 = F::cast_from(1.0_f64) / t37387;
    let t37389 = t18 * t37388;
    let t37391 = -F::cast_from(24.0_f64) * t7742 + F::cast_from(24.0_f64) * t37389;
    (t37382, t37383, t37385, t37386, t37391)
}
