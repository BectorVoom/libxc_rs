//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1464/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1464(t1063: f64, t11986: f64, t247: f64, t6096: f64, t1086: f64, t6343: f64, t994: f64, t19462: f64, t3286: f64, t3298: f64, t6235: f64, t3316: f64) -> (f64, f64, f64, f64, f64) {
    let t67575 = t1063 * t247 * t11986 * t6096;
    let t67652 = t994 * t1086 * t6343;
    let t67714 = t19462 * t3286;
    let t67725 = t6235 * t3298;
    let t67790 = t6235 * t3316;
    (t67575, t67652, t67714, t67725, t67790)
}
