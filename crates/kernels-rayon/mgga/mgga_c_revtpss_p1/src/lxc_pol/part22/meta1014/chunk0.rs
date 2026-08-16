//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3491/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3491(t11710: f64, t19730: f64, t3091: f64, t11672: f64, t11875: f64, t15604: f64, t15716: f64, t19572: f64, t19645: f64, t19731: f64, t247: f64, t3116: f64, t3117: f64, t42176: f64, t53407: f64, t53413: f64, t53416: f64, t53422: f64, t53427: f64, t53431: f64, t53433: f64, t65071: f64) -> f64 {
    let t65738 = t3091 * t11710 * t19730;
    let t65753 = -0.3811023832717309953e-3_f64 * t53407 - 0.28582678745379824648e-3_f64 * t53413 + 0.57165357490759649296e-3_f64 * t53416 - 0.95275595817932748826e-4_f64 * t42176 - 0.3811023832717309953e-3_f64 * t53422 - 0.30488190661738479624e-2_f64 * t11672 * t19731 + 0.3811023832717309953e-3_f64 * t65738 - 0.12862205435420921092e-2_f64 * t15716 * t247 * t3116 * t65071 + 0.96545937095505185476e-2_f64 * t53427 - 0.3811023832717309953e-3_f64 * t53431 + 0.3811023832717309953e-3_f64 * t53433 - 0.15244095330869239812e-2_f64 * t11672 * t19645 + 0.42874018118069736972e-3_f64 * t11875 * t3117 * t19572 * t15604;
    t65753
}
