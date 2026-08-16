//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1234/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1234(t1279: f64, t1281: f64, t1851: f64, t1853: f64, t19023: f64, t19037: f64, t19041: f64, t19044: f64, t19047: f64, t3403: f64, t3407: f64, t3410: f64, t547: f64, t548: f64, t5947: f64, t5954: f64, t5957: f64) -> f64 {
    let t19050 = 12.0_f64 * t1279 * t5954 + 6.0_f64 * t1279 * t5957 + 6.0_f64 * t1281 * t5947 + 6.0_f64 * t1851 * t3407 + 3.0_f64 * t1851 * t3410 + 3.0_f64 * t1853 * t3403 + t19023 * t548 + 6.0_f64 * t19037 * t547 + 12.0_f64 * t19041 * t547 + 6.0_f64 * t19044 * t547 + 3.0_f64 * t19047 * t547;
    t19050
}
