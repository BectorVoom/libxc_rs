//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1462/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1462(t11211: f64, t11213: f64, t11314: f64, t11317: f64, t14702: f64, t14708: f64, t14713: f64, t14759: f64, t14779: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64, t14796: f64, t14799: f64, t14802: f64, t14805: f64, t15072: f64, t15074: f64, t15091: f64, t15094: f64, t15115: f64) -> f64 {
    let t15117 = -t11314 - t11317 + 0.22954444444444444444e0_f64 * t14702 - t15072 + 0.516475e0_f64 * t14708 - t15074 + 0.104195e0_f64 * t14713 + 0.3529725e1_f64 * t14759 + 0.23154444444444444444e0_f64 * t11211 + 0.23154444444444444444e-1_f64 * t11213 + t15091 + 0.46308888888888888889e-1_f64 * t14779 - t15094 - 0.69463333333333333334e-1_f64 * t14784 - 0.34731666666666666667e-1_f64 * t14787 - 0.20839e0_f64 * t14790 + 0.41678e0_f64 * t14793 + 0.20839e0_f64 * t14796 + 0.62517e0_f64 * t14799 + 0.264729375e1_f64 * t14802 - 0.157790625e0_f64 * t14805 + t15115;
    t15117
}
