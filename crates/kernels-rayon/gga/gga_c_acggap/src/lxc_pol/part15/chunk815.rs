//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 815/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk815(t2146: f64, t2241: f64, t464: f64, t557: f64, t8123: f64, t8311: f64, t8314: f64, t8316: f64, t8319: f64, t8330: f64, t8332: f64, t8339: f64, t9003: f64, t9381: f64, t9386: f64, t9391: f64, t9397: f64, t9399: f64) -> f64 {
    let t9401 = 0.65854491829355115987e0_f64 * t8123 - 0.8673628188205199462e0_f64 * t8311 + 0.8673628188205199462e0_f64 * t8314 - 0.65854491829355115987e0_f64 * t9381 + 0.65854491829355115987e0_f64 * t8319 + 0.4336814094102599731e0_f64 * t2146 * t9386 - 0.65854491829355115987e0_f64 * t8316 * t557 + t8330 - 0.65854491829355115987e0_f64 * t9391 * t464 - 0.65854491829355115987e0_f64 * t8332 + 0.4336814094102599731e0_f64 * t9003 * t2241 - t8339 + 0.65854491829355115987e0_f64 * t9397 - 0.65854491829355115987e0_f64 * t9399;
    t9401
}
