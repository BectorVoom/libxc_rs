//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1852/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1852(t2434: f64, t371: f64, t373: f64, t367: f64, t3123: f64, t3168: f64, t3124: f64, t3173: f64, t1065: f64, t675: f64) -> (f64, f64, f64, f64, f64) {
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4_f64 * t367 * t11970;
    let t11977 = t3123 * t3168;
    let t11980 = t3124 * t3173;
    let t11986 = t675 * t1065;
    (t11970, t11972, t11977, t11980, t11986)
}
