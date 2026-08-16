//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 779/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk779(t1375: f64, t1386: f64, t2092: f64, t3758: f64, t3882: f64, t568: f64, t6893: f64, t6904: f64, t6909: f64, t7174: f64, t7176: f64, t7179: f64, t7192: f64, t7194: f64, t7199: f64, t7214: f64) -> f64 {
    let t7216 = -t7174 - 0.3289868133696452873e-1_f64 * t6893 - t7176 + 0.16449340668482264365e-1_f64 * t6904 - 0.16449340668482264365e-1_f64 * t6909 + t7179 * t568 + t7192 * t568 - t7194 * t1386 - t3758 * t2092 - t3882 * t2092 + 2.0_f64 * t1375 * t7199 - t1375 * t7214;
    t7216
}
