//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 972/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk972(t11270: f64, t8450: f64, t2923: f64, t5589: f64, t674: f64, t2906: f64, t3635: f64, t1736: f64, t1971: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t11271 = t11270 * t8450;
    let t11273 = t2923 * t674 * t5589;
    let t11274 = t11271 * t11273;
    let t11276 = t2906 * t3635;
    let t11301 = t1736 * pi;
    let t11302 = t1971 * t11301;
    (t11273, t11274, t11276, t11302)
}
