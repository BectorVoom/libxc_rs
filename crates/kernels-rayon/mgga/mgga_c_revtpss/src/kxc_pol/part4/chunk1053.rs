//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1053/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1053(t2434: f64, t371: f64, t373: f64, t367: f64, t3123: f64, t3168: f64, t3124: f64, t3173: f64, t1065: f64, t675: f64, t247: f64, t906: f64) -> (f64, f64, f64, f64, f64) {
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4_f64 * t367 * t11970;
    let t11977 = t3123 * t3168;
    let t11980 = t3124 * t3173;
    let t11986 = t675 * t1065;
    let t11988 = t247 * t11986 * t906;
    (t11972, t11977, t11980, t11986, t11988)
}
