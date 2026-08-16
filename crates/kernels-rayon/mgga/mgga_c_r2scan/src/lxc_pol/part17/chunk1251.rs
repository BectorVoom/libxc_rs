//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1251/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1251(t12811: f64, t498: f64, t3275: f64, t3352: f64, t3579: f64, t42234: f64, t11506: f64, t42318: f64, t3719: f64, t983: f64, t11002: f64, t3269: f64) -> (f64, f64, f64, f64, f64) {
    let t44555 = t498 * t12811;
    let t44558 = t3275 * t44555 * t3352 / 4.0_f64;
    let t44560 = t3579 * t42234 / 2.0_f64;
    let t44562 = 3.0_f64 / 2.0_f64 * t11506 * t42318;
    let t44563 = t3719 * t983;
    let t44564 = t11002 * t44563;
    let t44566 = 5.0_f64 / 8.0_f64 * t3269 * t44564;
    (t44555, t44558, t44560, t44562, t44566)
}
