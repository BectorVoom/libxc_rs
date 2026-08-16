//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 640/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk640(t125: f64, t1458: f64, t144: f64, t667: f64, t101: f64, t1474: f64, t122: f64, t1572: f64, t1971: f64, t457: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3938 = t1458 * t125;
    let t3940 = t667 * t144;
    let t3945 = t1474 * t101;
    let t3946 = t1572 * t122;
    let t3948 = t1971 * t144;
    let t3949 = t521 * t457;
    (t3938, t3940, t3945, t3946, t3948, t3949)
}
