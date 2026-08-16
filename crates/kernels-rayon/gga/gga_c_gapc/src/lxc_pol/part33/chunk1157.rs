//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1157/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1157(t3691: f64, t9099: f64, t11566: f64, t5252: f64, t128: f64, t1643: f64, t5248: f64, t671: f64, t3664: f64, t9294: f64, t11578: f64, t11579: f64, t1928: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t34344 = t3691 * t9099;
    let t34346 = t5252 * t11566;
    let t34351 = t1643 * t128 * t671 * pi * t5248;
    let t34353 = t3664 * t9294;
    let t34356 = t11578 * t11579 * t1928;
    (t34344, t34346, t34351, t34353, t34356)
}
