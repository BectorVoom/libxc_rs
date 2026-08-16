//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 344/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk344(t1352: f64, t384: f64, t527: f64, t935: f64, t1: f64, t483: f64, t283: f64, t1279: f64, t1280: f64, t659: f64, t684: f64, t693: f64, t700: f64, t711: f64, t714: f64, t753: f64, t757: f64, t805: f64, t809: f64) -> (f64, f64, f64, f64) {
    let t1353 = t384 * t1352;
    let t1355 = t935 * t527;
    let t1357 = t483 * t1;
    let t1358 = t1357 * t283;
    let t1359 = 0.18311447306006545054e-3_f64 * t1358;
    let t1360 = t659 - t684 - t693 + t700 - t1279 - t1280 + t711 - t714 - t1359 + t805 - t757 + t809 - t753;
    (t1353, t1355, t1357, t1360)
}
