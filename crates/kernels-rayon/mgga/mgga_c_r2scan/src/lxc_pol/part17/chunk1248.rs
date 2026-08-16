//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1248/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1248(t3262: f64, t3472: f64, t42919: f64, t11523: f64, t12033: f64, t1115: f64, t2530: f64, t3270: f64, t3579: f64, t11338: f64, t12567: f64, t3465: f64, t43717: f64) -> (f64, f64, f64, f64, f64) {
    let t44524 = 15.0_f64 / 8.0_f64 * t3262 * t3472 * t42919;
    let t44526 = t11523 * t12033 / 2.0_f64;
    let t44530 = t3579 * t3270 * t1115 * t2530 / 2.0_f64;
    let t44532 = t12567 * t11338 / 4.0_f64;
    let t44535 = 3.0_f64 / 4.0_f64 * t3262 * t3465 * t43717;
    (t44524, t44526, t44530, t44532, t44535)
}
