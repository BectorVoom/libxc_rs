//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1412/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1412(t13363: f64, t13419: f64, t10298: f64, t10301: f64, t10309: f64, t13267: f64, t13269: f64, t13272: f64, t13283: f64, t13286: f64, t13289: f64, t1497: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t4173: f64, t4178: f64, t4241: f64, t603: f64, t644: f64, t91: f64) -> (f64, f64) {
    let t13420 = t13363 + t13419;
    let t13423 = -4.0_f64 * t10298 * t1497 + 40.0_f64 * t10301 * t4178 - 120.0_f64 * t10309 * t13283 + t13267 * t91 - 8.0_f64 * t13269 * t644 + 20.0_f64 * t13272 * t2248 + 40.0_f64 * t13286 * t2247 + 20.0_f64 * t13289 * t2247 - 4.0_f64 * t13420 * t603 - 8.0_f64 * t2242 * t4241 - 4.0_f64 * t2315 * t4173;
    (t13420, t13423)
}
