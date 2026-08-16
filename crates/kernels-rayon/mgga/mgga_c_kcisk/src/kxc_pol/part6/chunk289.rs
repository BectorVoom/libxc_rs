//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 289/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk289(t442: f64, t451: f64, t1413: f64, t484: f64, t1414: f64, t492: f64, t1161: f64, t512: f64, t507: f64, t1184: f64, t515: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1472 = t451 * t442;
    let t1486 = t484 * t1413;
    let t1487 = t1486 * sigma0;
    let t1504 = t1414 * t492;
    let t1522 = 0.17123333333333333333e-1_f64 * t1161;
    let t1527 = t512 * t512;
    let t1528 = 1.0_f64 / t1527;
    let t1529 = t507 * t1528;
    let t1531 = 0.516475e0_f64 * t1161;
    let t1534 = 0.104195e0_f64 * t1184;
    let t1537 = 1.0_f64 / t515;
    (t1472, t1486, t1487, t1504, t1522, t1527, t1528, t1529, t1531, t1534, t1537)
}
