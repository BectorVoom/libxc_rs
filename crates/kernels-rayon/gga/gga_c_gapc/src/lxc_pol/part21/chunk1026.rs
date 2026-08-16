//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1026/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1026(t640: f64, t6939: f64, t102: f64, t2404: f64, t2446: f64, t2254: f64, t830: f64, t122: f64, t6924: f64, t2299: f64, t2530: f64, t6851: f64, t768: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23305 = t6939 * t640;
    let t23343 = t2446 * t102 * t2404;
    let t23466 = t830 * t2254;
    let t23523 = t6924 * t122;
    let t23579 = t2530 * t102 * t2299;
    let t23608 = t768 * t6851;
    (t23305, t23343, t23466, t23523, t23579, t23608)
}
