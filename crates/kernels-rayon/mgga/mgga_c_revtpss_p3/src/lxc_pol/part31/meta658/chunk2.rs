//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2224/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2224(t21663: f64, t607: f64, t13272: f64, t28126: f64, t2247: f64, t29524: f64, t38: f64, t5868: f64, t644: f64, t77: f64, t101320: f64, t1928: f64, t28127: f64, t28133: f64, t28138: f64, t28141: f64, t29526: f64, t29529: f64, t29533: f64, t6958: f64, t6960: f64, t6963: f64, t7706: f64, t7716: f64, t7720: f64) -> f64 {
    let t108769 = t21663 * t607;
    let t108772 = t13272 * t28126;
    let t108782 = t2247 * t38 * t29524;
    let t108792 = t77 * t5868 * t644;
    let t108799 = t108769 * t1928 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t108772 * t6960 + 2.0_f64 / 3.0_f64 * t28141 * t7716 + 5.0_f64 / 3.0_f64 * t28138 * t28133 + 2.0_f64 / 3.0_f64 * t28141 * t7720 + 5.0_f64 / 6.0_f64 * t108782 * t6960 + t6963 * t29526 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t28127 * t28133 + 2.0_f64 / 3.0_f64 * t6963 * t29529 + 5.0_f64 / 6.0_f64 * t6958 * t108792 + t6963 * t29533 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t101320 * t7706;
    t108799
}
