//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1307/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1307(t27924: f64, t303: f64, t3233: f64, t13174: f64, t4994: f64, t7718: f64, t1020: f64, t13137: f64, t10470: f64, t13113: f64, t3198: f64, t355: f64) -> (f64, f64, f64, f64) {
    let t96042 = t303 * t27924 * t3233;
    let t96045 = t4994 * t7718 * t13174;
    let t96048 = t1020 * t7718 * t13137;
    let t96052 = t10470 * t3198 * t355 * t13113;
    (t96042, t96045, t96048, t96052)
}
