//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 943/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk943(t2095: f64, t31191: f64, t2035: f64, t420: f64, t30059: f64, t7544: f64, t7676: f64, t1095: f64, t30572: f64, t30573: f64, t7458: f64, t1988: f64, t7689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31489 = t2095 * t31191;
    let t31491 = t2035 * t420;
    let t31492 = t31491 * t30059;
    let t31494 = t7676 * t7544;
    let t31495 = 0.28303283060643736861e-2_f64 * t31494;
    let t31498 = t30572 * t7458 * t1095 * t30573;
    let t31499 = 0.62896184579208304135e-3_f64 * t31498;
    let t31501 = t1988 * t7689;
    (t31489, t31491, t31492, t31495, t31499, t31501)
}
