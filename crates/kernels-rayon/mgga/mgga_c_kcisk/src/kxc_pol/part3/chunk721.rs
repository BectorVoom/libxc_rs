//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 721/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk721(t11208: f64, t1896: f64, t1901: f64, t4971: f64, t654: f64, t11154: f64, t1800: f64, t1869: f64, t4597: f64, t642: f64, t1757: f64, t3290: f64) -> (f64, f64, f64, f64, f64) {
    let t11209 = t11208 * t1896;
    let t11211 = t11208 * t1901;
    let t11213 = t654 * t4971;
    let t11214 = t11213 * t11154;
    let t11215 = t1800 * t11214;
    let t11216 = t1869 * t11215;
    let t11218 = t642 * t4597;
    let t11219 = t3290 * t1757;
    (t11209, t11211, t11216, t11218, t11219)
}
