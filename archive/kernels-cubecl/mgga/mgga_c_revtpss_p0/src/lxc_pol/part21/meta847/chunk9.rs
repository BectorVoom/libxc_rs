//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3184/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3184<F: Float>(t300: F, t57943: F, t57967: F, t58004: F, t58250: F, t58275: F, t58315: F, t58465: F, t58654: F, t16677: F, t3531: F, t16685: F) -> (F, F, F) {
    let t58658 = t300 * (t57943 + t57967 + t58004 + t58250 + t58275 + t58315 + t58465 + t58654);
    let t58660 = F::cast_from(0.70178683471615754484e1_f64) * t3531 * t16677;
    let t58662 = F::cast_from(0.51947577317044391277e2_f64) * t3531 * t16685;
    (t58658, t58660, t58662)
}
