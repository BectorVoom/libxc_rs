//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1087/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1087(t4978: f64, t5462: f64, t1403: f64, t1672: f64, t3005: f64, t5216: f64, t1784: f64, t1908: f64, t1911: f64, t1845: f64, t186: f64, t1803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t19677 = t5462 * t4978;
    let t19686 = t1672 * t1403;
    let t19765 = t3005 * t5216;
    let t19771 = pi * t1784 * t1908 * t1911;
    let t19844 = t1845 * t186;
    let t19902 = t1803 * t186;
    (t19677, t19686, t19765, t19771, t19844, t19902)
}
