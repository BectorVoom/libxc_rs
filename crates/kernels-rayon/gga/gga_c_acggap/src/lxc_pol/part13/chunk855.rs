//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 855/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk855(t30055: f64, t19: f64, t3220: f64, t336: f64, t3116: f64, t368: f64, t1980: f64, t1998: f64, t3732: f64, t151: f64, t177: f64, t3558: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30056 = 0.15724046144802076034e-3_f64 * t30055;
    let t30058 = t3220 * t19 * t336;
    let t30059 = t368 * t3116;
    let t30061 = t1980 * t30058 * t30059;
    let t30073 = t1998 * t3732;
    let t30077 = t151 * t587 * t3558 * t177;
    (t30056, t30058, t30059, t30061, t30073, t30077)
}
