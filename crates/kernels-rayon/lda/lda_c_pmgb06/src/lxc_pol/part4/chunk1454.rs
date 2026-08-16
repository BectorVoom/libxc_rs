//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1454/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1454(t8279: f64, t11307: f64, t11310: f64, t11317: f64, t11304: f64, t11313: f64, t8266: f64, t8282: f64, t8285: f64, t8287: f64, t8291: f64, t8293: f64, t8295: f64) -> (f64, f64, f64, f64, f64) {
    let t18634 = 3.8973666666666666_f64 * t8279;
    let t18637 = 3.8973666666666666_f64 * t11307;
    let t18638 = 2.5982444444444446_f64 * t11310;
    let t18640 = 5.196488888888889_f64 * t11317;
    let t18641 = -t8266 + t18634 - 1.95872_f64 * t8282 + t8285 + t8287 + t8291 + t8293 - t8295 + 7.83488_f64 * t11304 - t18637 - t18638 - 2.0_f64 * t11313 + t18640;
    (t18634, t18637, t18638, t18640, t18641)
}
