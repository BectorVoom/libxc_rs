//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 866/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk866(t10373: f64, t1058: f64, t3206: f64, t731: f64, t283: f64, t492: f64, t3193: f64, t2786: f64, t282: f64, t61: f64, t3189: f64, t132: f64, t3186: f64) -> (f64, f64, f64, f64, f64) {
    let t10374 = t10373 * t1058;
    let t10376 = t731 * t3206;
    let t10378 = t492 * t283;
    let t10379 = t10378 * t3193;
    let t10381 = t2786 * t282;
    let t10382 = t61 * t10381;
    let t10383 = t10382 * t3189;
    let t10385 = t132 * t3186;
    (t10374, t10376, t10379, t10383, t10385)
}
