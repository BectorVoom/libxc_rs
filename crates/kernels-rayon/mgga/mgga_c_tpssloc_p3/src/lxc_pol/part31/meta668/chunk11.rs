//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1976/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1976(t10109: f64, t7841: f64, t13065: f64, t13463: f64, t1528: f64, t17052: f64, t17090: f64, t2054: f64, t24305: f64, t25168: f64, t26703: f64, t26713: f64, t4147: f64, t4272: f64, t4301: f64, t5658: f64, t59498: f64, t7092: f64, t7107: f64, t7830: f64, t7842: f64, t85101: f64, t87779: f64, t92846: f64, t92847: f64, t92862: f64, t92866: f64, t92872: f64, t98921: f64, t98923: f64, t98927: f64) -> f64 {
    let t101551 = t10109 * t7841;
    let t101569 = -t85101 - t92846 - t24305 * t5658 + 4.0_f64 * t4147 * t26703 - 2.0_f64 * t13463 * t7842 - 2.0_f64 * t59498 * t2054 + t92862 - 12.0_f64 * t25168 * t101551 * t4272 - 2.0_f64 * t26713 * t4301 + 0.3289868133696452873e-1_f64 * t87779 - 2.0_f64 * t92847 * t1528 + 4.0_f64 * t13065 * t7830 - t92866 - t17090 * t7107 - t17052 * t7107 + 2.0_f64 * t17052 * t7092 + t92872 + 0.76763589786250567037e-1_f64 * t98921 - 0.76763589786250567037e-1_f64 * t98923 + 0.16449340668482264365e-1_f64 * t98927;
    t101569
}
