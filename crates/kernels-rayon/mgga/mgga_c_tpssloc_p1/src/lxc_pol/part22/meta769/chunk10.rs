//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2620/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2620(t20246: f64, t972: f64, t1198: f64, t15740: f64, t18364: f64, t3447: f64, t45250: f64, t53249: f64, t53322: f64, t53434: f64, t53440: f64, t53453: f64, t53490: f64, t6192: f64, t66571: f64, t66575: f64, t66597: f64, t66599: f64, t68513: f64) -> (f64, f64) {
    let t73113 = t20246 * t972;
    let t73126 = -t45250 + t66571 / 216.0_f64 - t53434 + t66575 / 108.0_f64 - 5.0_f64 / 1296.0_f64 * t53440 - t53453 + 77.0_f64 / 486.0_f64 * t73113 * t1198 - t66597 / 1152.0_f64 + t66599 / 216.0_f64 - 7.0_f64 / 216.0_f64 * t3447 * t53249 * t68513 - 5.0_f64 / 162.0_f64 * t53490 + 5.0_f64 / 4608.0_f64 * t15740 * t18364 - t53322 * t6192 / 768.0_f64;
    (t73113, t73126)
}
