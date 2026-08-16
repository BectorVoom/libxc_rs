//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 890/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk890(t435: f64, t879: f64, t1163: f64, t1165: f64, t3176: f64, t3171: f64, t3375: f64, t1159: f64, t3242: f64, t1162: f64, t1167: f64, t1172: f64, t12726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13232 = t435 * t879;
    let t13235 = t1163 * t1165 * t13232 * t3176;
    let t13253 = t3375 * t3171;
    let t13259 = t3242 * t1159;
    let t13260 = t13259 * t1162;
    let t13261 = t13260 * t1167;
    let t13263 = t12726 * t1172;
    (t13232, t13235, t13253, t13259, t13260, t13261, t13263)
}
