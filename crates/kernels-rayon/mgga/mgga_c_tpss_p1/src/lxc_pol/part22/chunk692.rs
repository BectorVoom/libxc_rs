//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 692/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk692(t219: f64, t3300: f64, t3313: f64, t1246: f64, t73: f64, t3245: f64, t1228: f64, t3234: f64, t1226: f64, t1229: f64, t516: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t3315 = (t3300 + t3313) * t219;
    let t3319 = t73 * t1246;
    let t3320 = t3319 * t3245;
    let t3323 = t1228 * t3234;
    let t3326 = 6.0_f64 * t1226 * t1229 - t3315 * t518 - 12.0_f64 * t3320 * t516 + 3.0_f64 * t3323 * t516;
    (t3315, t3320, t3323, t3326)
}
