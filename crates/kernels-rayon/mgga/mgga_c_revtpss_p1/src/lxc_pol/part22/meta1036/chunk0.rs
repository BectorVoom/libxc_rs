//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3623/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3623(t16682: f64, t5192: f64, t20652: f64, t44012: f64, t12227: f64, t20651: f64, t3427: f64, t3385: f64, t44091: f64, t44093: f64, t6438: f64, t5219: f64, t5412: f64) -> (f64, f64, f64, f64, f64) {
    let t68631 = 0.23392894490538584828e1_f64 * t5192 * t16682;
    let t68633 = 0.1034520258385468006e4_f64 * t44012 * t20652;
    let t68636 = 0.51726012919273400301e3_f64 * t12227 * t20651 * t3427;
    let t68640 = 0.24955700379505800916e5_f64 * t44091 * t6438 * t44093 * t3385;
    let t68658 = t5219 * t5412;
    (t68631, t68633, t68636, t68640, t68658)
}
