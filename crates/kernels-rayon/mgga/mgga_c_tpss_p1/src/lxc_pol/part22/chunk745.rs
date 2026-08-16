//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 745/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk745(t1429: f64, t876: f64, t1437: f64, t884: f64, t2455: f64, t2513: f64, t2557: f64, t2564: f64, t3746: f64, t3751: f64, t3756: f64, t3760: f64, t3774: f64, t3782: f64, t3790: f64, t3792: f64, t3795: f64, t3798: f64, t3801: f64, t3804: f64) -> (f64, f64, f64) {
    let t3822 = t1429 * t876;
    let t3827 = t1437 * t884;
    let t3844 = -0.17648625e1_f64 * t3774 + 0.3529725e1_f64 * t3782 + t2557 + 0.17215833333333333333e0_f64 * t2455 + 0.17215833333333333333e0_f64 * t3746 - 0.34431666666666666667e0_f64 * t3751 + 0.103295e1_f64 * t3756 - 0.516475e0_f64 * t3760 + 0.31558125e0_f64 * t3790 + 0.6311625e0_f64 * t3792 + t2564 + 0.69463333333333333333e-1_f64 * t2513 + 0.69463333333333333333e-1_f64 * t3795 - 0.34731666666666666667e-1_f64 * t3798 + 0.20839e0_f64 * t3801 - 0.104195e0_f64 * t3804;
    (t3822, t3827, t3844)
}
