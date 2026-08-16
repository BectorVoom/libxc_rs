//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 737/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk737(t4605: f64, t8504: f64, t8381: f64, t3137: f64, t6: f64, t101: f64, t4050: f64, t1462: f64, t4055: f64, t568: f64, t8415: f64, t466: f64) -> (f64, f64, f64, f64, f64) {
    let t8505 = t8504 * t4605;
    let t8506 = t8505 * t8381;
    let t8508 = t6 * t3137;
    let t8509 = t8508 * t101;
    let t8510 = t8509 * t4050;
    let t8511 = t1462 * t4055;
    let t8512 = t8510 * t8511;
    let t8514 = t8415 * t568;
    let t8515 = t466 * t8514;
    (t8506, t8508, t8510, t8512, t8515)
}
