//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 635/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk635(t339: f64, t63: f64, t67: f64, t378: f64, t4143: f64, t3951: f64, t9: f64, t403: f64, t3936: f64, t1310: f64, t398: f64, t1173: f64, t476: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6141 = t339 * t63 * t67;
    let t6142 = t378 * t4143;
    let t6174 = t9 * t3951;
    let t6175 = t6174 * t403;
    let t6183 = t3936 * t403;
    let t6204 = t1310 * t398;
    let t6256 = t476 * t1173;
    (t6141, t6142, t6174, t6175, t6183, t6204, t6256)
}
