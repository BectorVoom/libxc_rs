//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2211/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2211(t10309: f64, t29411: f64, t60224: f64, t7565: f64, t28150: f64, t7575: f64, t101156: f64, t101337: f64, t25120: f64, t25159: f64, t25162: f64, t26749: f64, t26755: f64, t26792: f64, t28133: f64, t28147: f64, t29364: f64, t29367: f64, t29380: f64, t6963: f64, t7566: f64, t8144: f64, t92588: f64, t96827: f64) -> f64 {
    let t104203 = t10309 * t29411;
    let t104208 = t60224 * t7565;
    let t104215 = t7575 * t28150;
    let t104222 = t25120 * t8144 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6963 * t29364 + 2.0_f64 / 3.0_f64 * t6963 * t29367 + 5.0_f64 / 3.0_f64 * t26749 * t28133 - 5.0_f64 * t104203 * t25159 - 5.0_f64 * t26792 * t101337 - 5.0_f64 * t104208 * t25159 - 5.0_f64 / 3.0_f64 * t92588 * t29380 - 10.0_f64 * t96827 * t28147 - 10.0_f64 / 3.0_f64 * t25162 * t104215 + 5.0_f64 / 3.0_f64 * t26755 * t28133 + 5.0_f64 / 3.0_f64 * t7566 * t101156;
    t104222
}
