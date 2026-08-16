//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1210/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1210(t25749: f64, t8375: f64, t6688: f64, t7593: f64, t118971: f64, t1921: f64, t1052: f64, t1055: f64, t113314: f64, t113318: f64, t113619: f64, t119201: f64, t119232: f64, t119366: f64, t119393: f64, t1955: f64, t1956: f64, t23346: f64, t23394: f64, t25419: f64, t25731: f64, t25738: f64, t25801: f64, t25826: f64, t30788: f64, t30854: f64, t3169: f64, t3174: f64, t32909: f64, t32917: f64, t32961: f64, t32993: f64, t388: f64, t6687: f64, t6691: f64, t6704: f64, t7565: f64, t88145: f64, t986: f64, t990: f64) -> f64 {
    let t119407 = t8375 * t25749;
    let t119412 = t6688 * t7593;
    let t119420 = t1921 * t118971;
    let t119440 = 0.43864908449286038307e-1_f64 * t23346 * t32993 - t1052 * t1055 * (t119201 + t119232 + t119366 + t119393) - 0.16449340668482264365e-1_f64 * t6687 * t30854 * t25826 + 0.3289868133696452873e-1_f64 * t6687 * t30854 * t25738 - 0.54831135561607547883e-2_f64 * t113314 - 2.0_f64 * t88145 * t1956 + 0.16449340668482264365e-1_f64 * t6687 * t986 * t119407 - 0.14621636149762012769e-1_f64 * t113318 + 0.54831135561607547883e-2_f64 * t6687 * t119412 * t6691 + 4.0_f64 * t3169 * t32917 + t990 * t32961 * t388 + 0.16449340668482264365e-1_f64 * t6687 * t986 * t119420 - 6.0_f64 * t3169 * t32909 + 0.3289868133696452873e-1_f64 * t6687 * t6704 * t23394 * t25419 - 0.16449340668482264365e-1_f64 * t6687 * t113619 * t7565 + 0.54831135561607547883e-2_f64 * t6687 * t30788 * t25801 + 4.0_f64 * t1052 * t3174 * t1955 * t25731;
    t119440
}
