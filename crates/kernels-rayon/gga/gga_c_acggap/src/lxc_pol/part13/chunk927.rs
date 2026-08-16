//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 927/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk927(t7770: f64, t7799: f64, t1980: f64, t31032: f64, t7476: f64, t1988: f64, t7693: f64, t7658: f64, t1017: f64, t355: f64, t3300: f64, t7458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31168 = t7799 * t7770;
    let t31179 = t1980 * t7476 * t31032;
    let t31186 = t1988 * t7693;
    let t31188 = t1988 * t7658;
    let t31190 = t355 * t1017;
    let t31191 = t3300 * t31190;
    let t31193 = t1980 * t7458 * t31191;
    (t31168, t31179, t31186, t31188, t31190, t31191, t31193)
}
