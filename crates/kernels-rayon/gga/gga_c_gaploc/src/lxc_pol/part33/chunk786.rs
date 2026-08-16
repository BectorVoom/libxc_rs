//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 786/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk786(t326: f64, t7438: f64, t1: f64, t7284: f64, t2021: f64, t2717: f64, t773: f64, t2653: f64, t783: f64, t701: f64, t7258: f64, t1445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7439 = t326 * t7438;
    let t7442 = t7284 * t1;
    let t7443 = t2021 * t7442;
    let t7448 = t773 * t2717;
    let t7453 = t2653 * t783;
    let t7458 = t7258 * t701;
    let t7459 = t1445 * t7458;
    (t7439, t7442, t7443, t7448, t7453, t7459)
}
