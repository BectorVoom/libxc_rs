//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 922/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk922(t30727: f64, t7670: f64, t7676: f64, t7724: f64, t2056: f64, t7600: f64, t2074: f64, t30456: f64, t2035: f64, t420: f64, t7544: f64, t1095: f64, t30572: f64, t30573: f64, t7458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31470 = t30727 * t7670;
    let t31472 = t7676 * t7724;
    let t31477 = t7600 * t2056;
    let t31479 = t30456 * t2074;
    let t31491 = t2035 * t420;
    let t31494 = t7676 * t7544;
    let t31498 = t30572 * t7458 * t1095 * t30573;
    (t31470, t31472, t31477, t31479, t31491, t31494, t31498)
}
