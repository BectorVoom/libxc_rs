//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 510/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk510(t1254: f64, t4075: f64, t1232: f64, t346: f64, t360: f64, t4032: f64, t4007: f64, t4011: f64, t4015: f64, t4018: f64, t4021: f64, t1260: f64, t45: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4076 = t4075 * t1254;
    let t4079 = t1232 * t1232;
    let t4080 = 1.0_f64 / t4079;
    let t4081 = t346 * t4080;
    let t4082 = t360 * t360;
    let t4083 = 1.0_f64 / t4082;
    let t4084 = t4032 * t4083;
    let t4087 = 0.12361111111111111111e-1_f64 * t4007;
    let t4092 = t4087 + 0.61805555555555555556e-2_f64 * t4011 - 0.61805555555555555555e-2_f64 * t4015 + 0.18541666666666666667e-1_f64 * t4018 - 0.92708333333333333333e-2_f64 * t4021;
    let t4096 = t45 * t1260;
    (t4076, t4079, t4080, t4081, t4082, t4083, t4084, t4092, t4096)
}
