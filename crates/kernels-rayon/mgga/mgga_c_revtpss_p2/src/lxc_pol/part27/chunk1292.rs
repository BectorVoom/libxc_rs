//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1292/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1292(t1937: f64, t46126: f64, t49851: f64, t10416: f64, t6993: f64, t25081: f64, t7234: f64, t25083: f64, t2014: f64, t25089: f64, t25190: f64, t28167: f64, t49616: f64, t8717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95083 = 2.0_f64 * t46126 * t1937;
    let t95085 = 6.0_f64 * t49851 * t1937;
    let t95087 = 6.0_f64 * t10416 * t6993;
    let t95088 = t7234 * t25081;
    let t95090 = 18.0_f64 * t95088 * t25083;
    let t95096 = 9.0_f64 * t2014 * t25190 * t25089;
    let t95104 = 18.0_f64 * t28167 * t8717 * t49616;
    (t95083, t95085, t95087, t95090, t95096, t95104)
}
