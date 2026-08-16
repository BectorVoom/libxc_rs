//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1211/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1211(t11506: f64, t41344: f64, t12033: f64, t40276: f64, t3275: f64, t3472: f64, t42966: f64, t3579: f64, t41348: f64, t12052: f64, t12422: f64, t2867: f64, t41202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44091 = 3.0_f64 / 2.0_f64 * t11506 * t41344;
    let t44093 = t40276 * t12033 / 2.0_f64;
    let t44096 = 5.0_f64 / 8.0_f64 * t3275 * t3472 * t42966;
    let t44098 = t3579 * t41348 / 2.0_f64;
    let t44100 = t12422 * t12052 / 4.0_f64;
    let t44103 = t3275 * t41202 * t2867 / 2.0_f64;
    (t44091, t44093, t44096, t44098, t44100, t44103)
}
