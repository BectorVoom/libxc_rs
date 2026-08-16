//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 243/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk243(t264: f64, t281: f64, t259: f64, t67: f64, t852: f64, t10: f64, t142: f64, t260: f64, t261: f64, t116: f64) -> (f64, f64, f64, f64, f64) {
    let t265 = t264 < -0.66725e-1_f64;
    let t1099 = t281 * t281;
    let t1100 = 1.0_f64 / t1099;
    let t1101 = t259 * t1100;
    let t1102 = t67 * t852;
    let t1110 = piecewise3(t265, 0.0_f64, 10.0_f64 / 9.0_f64 * t260 * t1102 * t10 - 10.0_f64 / 27.0_f64 * t260 * t261 * t142);
    let t1111 = t1110 * t116;
    (t1099, t1100, t1101, t1102, t1111)
}
