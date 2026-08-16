//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 892/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk892(t6012: f64, t6517: f64, t6556: f64, t2363: f64, t937: f64, t410: f64, t919: f64, t2970: f64, t6417: f64, t6523: f64, t2370: f64, t2421: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6557 = t6012 * t6517;
    let t6558 = t6556 * t6557;
    let t6561 = t2363 * t937;
    let t6565 = t2363 * t410 * t919;
    let t6566 = t2970 * t6417;
    let t6569 = t6523 * t410;
    let t6570 = t6012 * t2370;
    let t6571 = t6556 * t6570;
    let t6574 = t914 * t2421;
    (t6558, t6561, t6565, t6566, t6569, t6571, t6574)
}
