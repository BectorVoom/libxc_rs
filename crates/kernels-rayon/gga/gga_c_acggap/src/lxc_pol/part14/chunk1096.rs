//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1096/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1096(t1426: f64, t1579: f64, t2297: f64, t598: f64, t535: f64, t8539: f64, t1980: f64, t38795: f64, t7476: f64, t2001: f64, t5950: f64, t1861: f64, t7605: f64) -> (f64, f64, f64, f64, f64) {
    let t39182 = t598 * t1426 * t1579 * t2297;
    let t39186 = t598 * t1426 * t535 * t8539;
    let t39189 = t1980 * t7476 * t38795;
    let t39192 = t2001 * t5950;
    let t39194 = t7605 * t1861;
    (t39182, t39186, t39189, t39192, t39194)
}
