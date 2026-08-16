//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1084/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1084(t19159: f64, t442: f64, t8139: f64, t2642: f64, t2763: f64, t2766: f64, t2315: f64, t7389: f64, t672: f64, t818: f64, t1087: f64, t2299: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t19161 = t8139 * t19159 * t442;
    let t19179 = pi * t2642 * t2763 * t2766;
    let t19196 = t7389 * t2315;
    let t19204 = t672 * t818;
    let t19210 = t1087 * t2299;
    (t19161, t19179, t19196, t19204, t19210)
}
