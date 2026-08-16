//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 699/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk699(t2253: f64, t3655: f64, t3539: f64, t604: f64, t1882: f64, t3324: f64, t3327: f64, t3320: f64, t3339: f64, t9065: f64, t8796: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12240 = 2.0_f64 / 3.0_f64 * t2253 * t3655;
    let t12277 = t3539 * t604;
    let t12306 = t1882 * t3324;
    let t12307 = t12306 / 27.0_f64;
    let t12308 = t1882 * t3327;
    let t12309 = 2.0_f64 / 27.0_f64 * t12308;
    let t12310 = t1882 * t3320;
    let t12311 = 2.0_f64 / 81.0_f64 * t12310;
    let t12327 = t1882 * t3339;
    let t12328 = t12327 / 27.0_f64;
    let t12343 = 4.0_f64 / 27.0_f64 * t9065;
    let t12346 = 4.0_f64 / 81.0_f64 * t8796;
    (t12240, t12277, t12306, t12307, t12308, t12309, t12310, t12311, t12327, t12328, t12343, t12346)
}
