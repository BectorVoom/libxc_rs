//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 580/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk580(t3248: f64, t943: f64, t2673: f64, t959: f64, t2610: f64, t935: f64, t2365: f64, t2033: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3250 = 0.64087718584518535698e-3_f64 * t943 * t3248;
    let t3275 = 0.29792074959875355558e-1_f64 * t2673 * t959;
    let t3280 = t2610 * t935;
    let t3281 = t2365 * t3280;
    let t3283 = 0.29792074959875355558e-1_f64 * t2033 * t3281;
    let t3294 = t935 * t123;
    let t3295 = t3294 * t883;
    (t3250, t3275, t3280, t3281, t3283, t3294, t3295)
}
