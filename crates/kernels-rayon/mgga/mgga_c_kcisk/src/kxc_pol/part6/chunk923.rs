//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 923/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk923(t28294: f64, t5290: f64, t5289: f64, t28256: f64, t5322: f64, t7429: f64, t28957: f64, t41: f64, t719: f64, t734: f64, t28324: f64, t5321: f64) -> (f64, f64, f64, f64) {
    let t29382 = t5290 * t28294;
    let t29383 = t5289 * t29382;
    let t29385 = t5322 * t28256;
    let t29386 = t7429 * t29385;
    let t29388 = t28957 * t41;
    let t29389 = t29388 * t719;
    let t29390 = t734 * t29389;
    let t29392 = t5322 * t28324;
    let t29393 = t5321 * t29392;
    (t29383, t29386, t29390, t29393)
}
