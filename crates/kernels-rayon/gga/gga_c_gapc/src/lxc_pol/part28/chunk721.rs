//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 721/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk721(t2957: f64, t8300: f64, t2937: f64, t4893: f64, t1268: f64, t991: f64, t1803: f64, t515: f64, t996: f64, t1504: f64, t493: f64, t1928: f64, t435: f64) -> (f64, f64, f64, f64, f64) {
    let t8301 = t2957 * t8300;
    let t8303 = t2937 * t4893;
    let t8304 = t2957 * t8303;
    let t8306 = t1268 * t991;
    let t8308 = t1803 * t515;
    let t8309 = t996 * t8308;
    let t8310 = t493 * t1504;
    let t8311 = t8309 * t8310;
    let t8313 = t435 * t1928;
    (t8301, t8304, t8306, t8311, t8313)
}
