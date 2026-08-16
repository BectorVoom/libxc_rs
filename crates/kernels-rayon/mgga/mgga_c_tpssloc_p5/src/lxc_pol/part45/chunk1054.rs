//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1054/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1054(t31781: f64, t580: f64, t1404: f64, t8646: f64, t2022: f64, t7240: f64, t31820: f64, t576: f64, t1395: f64, t8660: f64, t2029: f64, t7222: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t116014 = t31781 * t580;
    let t116021 = t8646 * t1404;
    let t116026 = t2022 * t7240;
    let t116028 = t576 * t31820;
    let t116032 = t1395 * t8660;
    let t116036 = t7222 * t2029;
    (t116014, t116021, t116026, t116028, t116032, t116036)
}
