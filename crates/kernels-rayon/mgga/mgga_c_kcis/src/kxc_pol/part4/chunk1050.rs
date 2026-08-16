//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1050/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1050(t1096: f64, t13261: f64, t1092: f64, t1773: f64, t3228: f64, t1131: f64, t3227: f64, t4807: f64, t9429: f64, t2855: f64, t4772: f64, t2861: f64, t4778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13262 = t1096 * t13261;
    let t13263 = t1092 * t13262;
    let t13265 = t1773 * t3228;
    let t13266 = t1131 * t13265;
    let t13267 = t3227 * t13266;
    let t13268 = t1092 * t13267;
    let t13270 = t9429 * t4807;
    let t13271 = 0.14739506172839506172e-2_f64 * t13270;
    let t13273 = t2855 * t4772;
    let t13274 = t1096 * t13273;
    let t13275 = t1092 * t13274;
    let t13277 = t2861 * t4778;
    (t13263, t13265, t13268, t13270, t13271, t13275, t13277)
}
