//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 873/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk873(t2902: f64, t6808: f64, t3244: f64, t291: f64, t467: f64, t787: f64, t2238: f64, t1055: f64, t876: f64, t3209: f64, t10105: f64, t1058: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10110 = t2902 * t6808;
    let t10111 = t10110 * t3244;
    let t10113 = t467 * t291;
    let t10114 = t10113 * t787;
    let t10115 = t2238 * t10114;
    let t10117 = t1055 * t876;
    let t10118 = t3209 * t10117;
    let t10120 = t10105 * t1058;
    (t10110, t10111, t10113, t10115, t10118, t10120)
}
