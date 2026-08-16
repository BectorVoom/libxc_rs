//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1052/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1052(t1174: f64, t4916: f64, t1714: f64, t3448: f64, t3451: f64, t3295: f64, t3464: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64) -> (f64, f64, f64, f64) {
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4920 = t4919 * t3451;
    let t4928 = -t3464 + t3295 / 9.0_f64 + t4770 / 9.0_f64 + t4773 / 18.0_f64 - t4776 / 3.0_f64 - t4779 / 6.0_f64;
    (t4917, t4919, t4920, t4928)
}
