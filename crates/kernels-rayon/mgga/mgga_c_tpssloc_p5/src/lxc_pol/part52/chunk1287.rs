//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1287/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1287(t23168: f64, t32819: f64, t234: f64, t7510: f64, t6552: f64, t6637: f64, t776: f64, t112951: f64, t1484: f64, t1888: f64, t232: f64, t6646: f64, t87567: f64) -> (f64, f64, f64, f64) {
    let t118744 = t23168 * t32819;
    let t118745 = 0.76763589786250567037e-1_f64 * t118744;
    let t118747 = t234 * t7510;
    let t118751 = 0.3289868133696452873e-1_f64 * t6552 * t6637 * t118747 * t776;
    let t118756 = 0.3289868133696452873e-1_f64 * t6552 * t6637 * t112951 * t1484;
    let t118760 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t87567 * t232;
    (t118745, t118751, t118756, t118760)
}
