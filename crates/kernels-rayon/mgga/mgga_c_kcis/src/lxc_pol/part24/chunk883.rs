//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 883/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk883(t278: f64, t19160: f64, t3202: f64, t4554: f64, t1646: f64, t1704: f64, t829: f64, t14408: f64, t14395: f64, t330: f64, t1003: f64, t14401: f64, t19107: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t288 = 0.0_f64 < t278;
    let t19161 = t3202 * t19160;
    let t19162 = t4554 * t19161;
    let t19164 = t1646 * t1704;
    let t19165 = t19164 * t829;
    let t19166 = t14408 * t19165;
    let t19171 = t14395 * t330;
    let t19173 = t19171 * t19164 * t1003;
    let t19176 = t14401 * t19165;
    let t19180 = piecewise3(t288, t19107, -t19107);
    (t19162, t19164, t19166, t19173, t19176, t19180)
}
