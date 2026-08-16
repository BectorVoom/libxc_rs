//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1428/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1428(t11352: f64, t3351: f64, t11344: f64, t11350: f64, t1136: f64, t1138: f64, t11415: f64, t11420: f64, t11441: f64, t1148: f64, t1156: f64, t3332: f64, t3333: f64, t3334: f64, t3357: f64, t3359: f64, t3360: f64, t43911: f64, t43997: f64, t44000: f64, t44002: f64, t44006: f64, t44072: f64, t44080: f64, t44082: f64, t44085: f64, t44089: f64, t44092: f64) -> f64 {
    let t44131 = t3351 * t11352;
    let t44138 = t43997 + t44000 - t44002 - t44006 - t44072 - t44080 - t44082 + t44085 + t44089 - t44092 + 36.0_f64 * t3357 * t3334 * t3351 - 8.0_f64 * t3332 * t1138 * t11344 - 0.11579025239058625248e4_f64 * t11420 * t3360 * t3351 + 0.3859675079686208416e3_f64 * t11415 * t11441 + 0.12865583598954028054e3_f64 * t3357 * t11344 * t3359 * t1136 + 0.12414243100625616072e5_f64 * t11350 * t44131 * t3333 + 0.5848223622634646207e0_f64 * t1148 * t43911 * t1156;
    t44138
}
