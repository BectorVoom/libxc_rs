//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1049/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1049(t1: f64, t2546: f64, t286: f64, t3: f64, t3074: f64, t786: f64, t10057: f64, t7191: f64, t21: f64, t3328: f64, t3787: f64, t15609: f64) -> (f64, f64, f64, f64) {
    let t29481 = t2546 * t3074 * t286 * t1 * t3 * t786;
    let t29516 = t10057 * t7191;
    let t29568 = t3787 * t3328 * t21;
    let t29571 = t3787 * t15609;
    (t29481, t29516, t29568, t29571)
}
