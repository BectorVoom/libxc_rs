//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1250/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1250(t30: f64, t4806: f64, t1288: f64, t1398: f64, t4802: f64, t33: f64, t4706: f64, t18246: f64, t21262: f64, t1364: f64, t1497: f64, t4701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21353 = t30 * t4806;
    let t21356 = t1288 * t1398;
    let t21359 = t30 * t4802;
    let t21485 = t33 * t4706;
    let t21492 = t18246 * t21262;
    let t21495 = t1497 * t1364;
    let t21499 = t33 * t4701;
    (t21353, t21356, t21359, t21485, t21492, t21495, t21499)
}
