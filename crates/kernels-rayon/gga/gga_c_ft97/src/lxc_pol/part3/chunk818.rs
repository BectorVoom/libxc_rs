//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 818/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk818(t16719: f64, t9049: f64, t446: f64, t15742: f64, t2205: f64, t15737: f64, t9327: f64, t15746: f64, t3281: f64, t3408: f64, t925: f64, t1969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16720 = t9049 * t16719;
    let t16721 = t446 * t16720;
    let t16723 = t2205 * t15742;
    let t16724 = t446 * t16723;
    let t16726 = t9327 * t15737;
    let t16727 = t446 * t16726;
    let t16729 = t2205 * t15746;
    let t16730 = t3281 * t16729;
    let t16732 = t925 * t3408;
    let t16733 = t1969 * t16732;
    (t16721, t16724, t16727, t16730, t16732, t16733)
}
