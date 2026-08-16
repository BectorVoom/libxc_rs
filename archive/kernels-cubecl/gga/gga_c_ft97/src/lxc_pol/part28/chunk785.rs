//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 785/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk785<F: Float>(t32664: F, t356: F, t461: F, t5925: F, t342: F, t630: F, t7302: F, t5842: F, t72: F, t1349: F, t1526: F, t1527: F, t2: F, t32658: F, t32663: F, t343: F, t5917: F, t5922: F, t7298: F, t7299: F) -> (F, F, F, F, F) {
    let t32665 = t356 * t32664;
    let t32670 = t461 * t5925;
    let t32675 = t342 * t630 * t7302 / F::cast_from(12.0_f64);
    let t32679 = t72 * t5842;
    let t32684 = (-t32658 * t7299 / F::cast_from(6.0_f64) + t32663 + t1349 * t32665 / F::cast_from(18.0_f64) + t1349 * t5922 / F::cast_from(3.0_f64) - t7298 * t32670 / F::cast_from(6.0_f64) - t32675 - t1526 * t1527 * t5917 / F::cast_from(12.0_f64) - t342 * t343 * t32679 / F::cast_from(4.0_f64)) * t2;
    (t32665, t32670, t32675, t32679, t32684)
}
