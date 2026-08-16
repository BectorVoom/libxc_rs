//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2450/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2450(t12046: f64, t15905: f64, t994: f64, t3114: f64, t42416: f64, t11652: f64, t3172: f64, t4837: f64, t1063: f64, t11986: f64, t247: f64, t2862: f64) -> (f64, f64, f64, f64) {
    let t42690 = t994 * t12046 * t15905;
    let t42695 = t3114 * t42416;
    let t42699 = t4837 * t3172 * t11652;
    let t42710 = t1063 * t247 * t11986 * t2862;
    (t42690, t42695, t42699, t42710)
}
