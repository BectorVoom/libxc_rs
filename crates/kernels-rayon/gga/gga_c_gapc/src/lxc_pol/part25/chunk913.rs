//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 913/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk913(t10102: f64, t1058: f64, t2153: f64, t996: f64, t3206: f64, t731: f64, t283: f64, t492: f64, t3193: f64, t2786: f64, t282: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10371 = t10102 * t1058;
    let t10373 = t996 * t2153;
    let t10374 = t10373 * t1058;
    let t10376 = t731 * t3206;
    let t10378 = t492 * t283;
    let t10379 = t10378 * t3193;
    let t10381 = t2786 * t282;
    let t10382 = t61 * t10381;
    (t10371, t10373, t10374, t10376, t10379, t10382)
}
