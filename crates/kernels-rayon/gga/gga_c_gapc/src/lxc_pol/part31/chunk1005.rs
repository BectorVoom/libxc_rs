//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1005/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1005(t116: f64, t612: f64, t144: f64, t3137: f64, t674: f64, t5059: f64, t641: f64, t1908: f64, t198: f64, t655: f64, t3163: f64, t3691: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11537 = t116 * t612;
    let t11539 = t3137 * t144 * t674;
    let t11540 = t11539 * t5059;
    let t11541 = t11537 * t11540;
    let t11543 = t116 * t641;
    let t11546 = t3137 * t198 * t1908 * t5059;
    let t11547 = t11543 * t11546;
    let t11549 = t116 * t655;
    let t11550 = t11549 * t11546;
    let t11552 = t3691 * t3163;
    (t11537, t11540, t11541, t11543, t11546, t11547, t11549, t11550, t11552)
}
