//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 868/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk868(t10360: f64, t284: f64, t10142: f64, t876: f64, t2902: f64, t932: f64, t1055: f64, t787: f64, t10102: f64, t1058: f64, t2153: f64, t996: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10361 = t284 * t10360;
    let t10363 = t10142 * t876;
    let t10364 = t284 * t10363;
    let t10366 = t2902 * t932;
    let t10367 = t1055 * t787;
    let t10368 = t10366 * t10367;
    let t10371 = t10102 * t1058;
    let t10373 = t996 * t2153;
    (t10361, t10364, t10366, t10368, t10371, t10373)
}
