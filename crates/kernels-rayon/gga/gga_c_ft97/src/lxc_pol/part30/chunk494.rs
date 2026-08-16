//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 494/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk494(t1526: f64, t2322: f64, t9483: f64, t2326: f64, t342: f64, t630: f64, t2427: f64, t677: f64, t322: f64, t668: f64, t693: f64, t226: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9485 = t1526 * t9483 * t2322;
    let t9488 = t342 * t630 * t2326;
    let t9533 = t677 * t2427;
    let t9567 = 1.0_f64 / t322 / t668;
    let t9680 = t693 * t693;
    let t9681 = 1.0_f64 / t9680;
    let t9682 = t226 * t9681;
    (t9485, t9488, t9533, t9567, t9680, t9681, t9682)
}
