//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 591/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk591(t25524: f64, t28: f64, t1586: f64, t6454: f64, t5508: f64, t432: f64, t984: f64, t5507: f64, t1308: f64, t3289: f64, t376: f64, t6456: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25525 = t28 * t25524;
    let t25528 = t1586 * t6454;
    let t25529 = t25528 * t5508;
    let t25530 = t28 * t25529;
    let t25533 = t984 * t432;
    let t25534 = t5507 * t25533;
    let t25535 = t28 * t25534;
    let t25538 = t1308 * t3289;
    let t25539 = t28 * t25538;
    let t25542 = t376 * t6456;
    (t25525, t25528, t25530, t25533, t25535, t25539, t25542)
}
