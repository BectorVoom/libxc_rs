//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 767/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk767(t701: f64, t7221: f64, t1901: f64, t1835: f64, t2571: f64, t2530: f64, t835: f64, t723: f64, t2580: f64, t161: f64, t2536: f64, t1854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7250 = t7221 * t701;
    let t7251 = t1901 * t7250;
    let t7254 = t2571 * t1835;
    let t7255 = t1901 * t7254;
    let t7258 = t835 * t2530;
    let t7259 = t7258 * t723;
    let t7260 = t2580 * t7259;
    let t7267 = t2536 * t161;
    let t7268 = t7267 * t1854;
    (t7250, t7251, t7254, t7255, t7258, t7259, t7260, t7268)
}
