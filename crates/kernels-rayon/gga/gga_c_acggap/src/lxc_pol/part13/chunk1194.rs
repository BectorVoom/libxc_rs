//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1194/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1194(t1413: f64, t7712: f64, t2310: f64, t7630: f64, t2001: f64, t4728: f64, t31849: f64, t13287: f64, t31195: f64, t33953: f64, t5270: f64, t15386: f64, t36323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36331 = t7712 * t1413;
    let t36332 = 0.85748036236139473944e-3_f64 * t36331;
    let t36333 = t7630 * t2310;
    let t36335 = t2001 * t4728;
    let t36340 = 0.15724046144802076034e-2_f64 * t31849;
    let t36344 = t31195 * t13287 * t33953 * t5270;
    let t36347 = t31195 * t15386 * t36323;
    (t36332, t36333, t36335, t36340, t36344, t36347)
}
