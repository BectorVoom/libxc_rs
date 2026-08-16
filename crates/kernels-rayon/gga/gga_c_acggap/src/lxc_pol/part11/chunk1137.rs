//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1137/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1137(t35608: f64, t7433: f64, t8787: f64, t1165: f64, t20433: f64, t2068: f64, t7351: f64, t31362: f64, t8956: f64, t525: f64, t839: f64, t604: f64, t7337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35609 = 0.20965394859736101378e-3_f64 * t35608;
    let t35610 = t7433 * t8787;
    let t35611 = 0.56606566121287473722e-2_f64 * t35610;
    let t35614 = t2068 * t1165 * t7351 * t20433;
    let t35616 = t31362 * t8956;
    let t35617 = 0.15724046144802076034e-2_f64 * t35616;
    let t35618 = t525 * t839;
    let t35621 = t7337 * t1165 * t604 * t35618;
    (t35609, t35611, t35614, t35617, t35618, t35621)
}
