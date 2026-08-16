//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1162/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1162(t109396: f64, t25904: f64, t25899: f64, t30278: f64, t686: f64, t72: f64, t94674: f64, t30295: f64, t7284: f64, t30282: f64, t25895: f64, t689: f64, t6919: f64, t7492: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t109397 = t25904 * t109396;
    let t109400 = t25899 * t109396;
    let t109403 = t30278 * t72 * t686;
    let t109404 = t94674 * t109403;
    let t109407 = t30295 * t72 * t686;
    let t109408 = t7284 * t109407;
    let t109412 = t30282 * t72 * t686;
    let t109413 = t25895 * t109412;
    let t109417 = t689 * t7492 * t6919;
    (t109397, t109400, t109403, t109404, t109407, t109408, t109412, t109413, t109417)
}
