//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 639/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk639(t1526: f64, t1529: f64, t7705: f64, t1533: f64, t342: f64, t630: f64, t360: f64, t18: f64, t23: f64, t7241: f64, t174: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7707 = t1526 * t7705 * t1529;
    let t7710 = t342 * t630 * t1533;
    let t7741 = t360 * t360;
    let t7742 = 1.0_f64 / t7741;
    let t7743 = t18 * t7742;
    let t7750 = t23 * t7241;
    let t7760 = 1.0_f64 / t174 / t358;
    (t7707, t7710, t7741, t7743, t7750, t7760)
}
