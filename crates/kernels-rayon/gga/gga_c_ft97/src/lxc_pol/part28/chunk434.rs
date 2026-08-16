//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 434/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk434(t165: f64, t6616: f64, t28: f64, t1058: f64, t1360: f64, t5855: f64, t925: f64, t2221: f64, t1017: f64, t1359: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6617 = t6616 * t165;
    let t6618 = t28 * t6617;
    let t6621 = t1360 * t1058;
    let t6622 = t28 * t6621;
    let t6626 = t5855 * t925;
    let t6627 = t2221 * t6626;
    let t6630 = t1359 * t1017;
    (t6617, t6618, t6621, t6622, t6626, t6627, t6630)
}
