//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 265/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk265(t1163: f64, t1248: f64, t1249: f64, t1227: f64, t1238: f64, t1240: f64, t1243: f64, t1247: f64, t360: f64) -> (f64, f64, f64) {
    let t1251 = t1248 * t1249 * t1163;
    let t1253 = 0.1898925e1_f64 * t1238 - t1240 - 0.29896666666666666667e0_f64 * t1227 + 0.3071625e0_f64 * t1243 - t1247 - 0.16431333333333333333e0_f64 * t1251;
    let t1254 = 1.0_f64 / t360;
    (t1251, t1253, t1254)
}
