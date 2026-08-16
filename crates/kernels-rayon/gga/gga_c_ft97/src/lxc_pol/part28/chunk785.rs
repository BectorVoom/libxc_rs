//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 785/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk785(t32664: f64, t356: f64, t461: f64, t5925: f64, t342: f64, t630: f64, t7302: f64, t5842: f64, t72: f64, t1349: f64, t1526: f64, t1527: f64, t2: f64, t32658: f64, t32663: f64, t343: f64, t5917: f64, t5922: f64, t7298: f64, t7299: f64) -> (f64, f64, f64, f64, f64) {
    let t32665 = t356 * t32664;
    let t32670 = t461 * t5925;
    let t32675 = t342 * t630 * t7302 / 12.0_f64;
    let t32679 = t72 * t5842;
    let t32684 = (-t32658 * t7299 / 6.0_f64 + t32663 + t1349 * t32665 / 18.0_f64 + t1349 * t5922 / 3.0_f64 - t7298 * t32670 / 6.0_f64 - t32675 - t1526 * t1527 * t5917 / 12.0_f64 - t342 * t343 * t32679 / 4.0_f64) * t2;
    (t32665, t32670, t32675, t32679, t32684)
}
