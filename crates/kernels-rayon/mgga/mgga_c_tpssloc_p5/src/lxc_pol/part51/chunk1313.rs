//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1313/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1313(t118709: f64, t1888: f64, t232: f64, t6646: f64, t7510: f64, t828: f64, t118690: f64, t25038: f64, t25248: f64, t776: f64, t30676: f64, t4119: f64, t6552: f64, t6637: f64) -> (f64, f64, f64, f64) {
    let t118710 = 0.82246703342411321825e-2_f64 * t118709;
    let t118715 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t7510 * t828 * t232;
    let t118719 = 0.9869604401089358619e-1_f64 * t25038 * t25248 * t118690 * t776;
    let t118725 = 0.3289868133696452873e-1_f64 * t6552 * t6637 * t30676 * t4119;
    (t118710, t118715, t118719, t118725)
}
