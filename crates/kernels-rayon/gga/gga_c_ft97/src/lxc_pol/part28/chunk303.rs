//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 303/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk303(t1009: f64, t3347: f64, t1995: f64, t1008: f64, t549: f64, t554: f64, t2007: f64, t929: f64, t120: f64, t383: f64, t3056: f64, t528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3348 = t3347 * t1009;
    let t3350 = t1995 * t1009;
    let t3355 = t549 * t1008;
    let t3356 = t3355 * t554;
    let t3359 = t2007 * t929;
    let t3360 = t120 * t383;
    let t3363 = t528 * t3056;
    (t3348, t3350, t3356, t3359, t3360, t3363)
}
