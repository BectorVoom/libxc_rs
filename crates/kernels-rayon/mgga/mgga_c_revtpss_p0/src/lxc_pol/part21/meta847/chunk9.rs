//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3184/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3184(t300: f64, t57943: f64, t57967: f64, t58004: f64, t58250: f64, t58275: f64, t58315: f64, t58465: f64, t58654: f64, t16677: f64, t3531: f64, t16685: f64) -> (f64, f64, f64) {
    let t58658 = t300 * (t57943 + t57967 + t58004 + t58250 + t58275 + t58315 + t58465 + t58654);
    let t58660 = 0.70178683471615754484e1_f64 * t3531 * t16677;
    let t58662 = 0.51947577317044391277e2_f64 * t3531 * t16685;
    (t58658, t58660, t58662)
}
