//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1181/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1181(t1279: f64, t1281: f64, t13265: f64, t13279: f64, t13283: f64, t13286: f64, t13289: f64, t1668: f64, t1670: f64, t3403: f64, t3407: f64, t3410: f64, t4549: f64, t4556: f64, t4559: f64, t547: f64, t548: f64) -> f64 {
    let t13292 = 12.0_f64 * t1279 * t4556 + 6.0_f64 * t1279 * t4559 + 6.0_f64 * t1281 * t4549 + t13265 * t548 + 6.0_f64 * t13279 * t547 + 12.0_f64 * t13283 * t547 + 6.0_f64 * t13286 * t547 + 3.0_f64 * t13289 * t547 + 6.0_f64 * t1668 * t3407 + 3.0_f64 * t1668 * t3410 + 3.0_f64 * t1670 * t3403;
    t13292
}
