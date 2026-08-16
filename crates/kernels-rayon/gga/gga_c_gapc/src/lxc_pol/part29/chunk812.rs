//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 812/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk812(t9438: f64, t9439: f64, t3340: f64, t3424: f64, t8998: f64, t933: f64, t2629: f64, t1081: f64, t2757: f64, t2573: f64, t3303: f64, t1092: f64, t2548: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9440 = t9438 * t9439;
    let t9442 = t3424 * t3340;
    let t9444 = t933 * t8998;
    let t9445 = t9444 * t2629;
    let t9447 = t1081 * t2757;
    let t9449 = t3303 * t2573;
    let t9451 = t1092 * t2548;
    (t9440, t9442, t9445, t9447, t9449, t9451)
}
