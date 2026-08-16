//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 718/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk718(t1956: f64, t2067: f64, t213: f64, t257: f64, t7067: f64, t7070: f64, t7387: f64, t7390: f64, t7399: f64, t7403: f64, t7409: f64, t7411: f64, t7415: f64, t7420: f64, t7424: f64, t887: f64) -> f64 {
    let t7427 = -t7387 + t7390 + 0.65854491829355115987e0_f64 * t213 * t7399 * t257 - 0.65854491829355115987e0_f64 * t7403 * t887 + t7409 - t7411 - 0.4336814094102599731e0_f64 * t7067 * t2067 + 0.8673628188205199462e0_f64 * t7070 * t7415 + 0.4336814094102599731e0_f64 * t7070 * t7420 - 0.4336814094102599731e0_f64 * t1956 * t7424;
    t7427
}
