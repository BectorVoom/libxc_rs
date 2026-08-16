//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1336/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1336(t105223: f64, t105232: f64, t105240: f64, t105269: f64, t105443: f64, t105466: f64, t105474: f64, t105508: f64, t105700: f64, t105723: f64, t1527: f64, t17092: f64, t1911: f64, t21033: f64, t21049: f64, t25188: f64, t25348: f64, t2718: f64, t28311: f64, t28431: f64, t40890: f64, t4147: f64, t5637: f64, t5658: f64, t7517: f64, t82219: f64, t855: f64, t86870: f64, t86903: f64, t87779: f64, t98117: f64, t98921: f64, t98923: f64, t98927: f64, t98932: f64) -> f64 {
    let t105726 = 0.24674011002723396547e-1_f64 * t87779 - 0.38381794893125283518e0_f64 * t86903 - 0.15626873635058151147e0_f64 * t86870 - t82219 + 12.0_f64 * t17092 * t7517 - 18.0_f64 * t4147 * t28311 + 6.0_f64 * t25348 * t5637 - 3.0_f64 * t25188 * t5658 + t105700 + t105466 + 0.11514538467937585055e0_f64 * t98932 + 0.24674011002723396548e-1_f64 * t98927 + 0.23029076935875170111e0_f64 * t98117 + t105723 + t105269 + 0.14804406601634037928e0_f64 * t105223 + t105443 + 0.49348022005446793095e-1_f64 * t105240 + 0.49348022005446793095e-1_f64 * t105474 + t105508 - 0.19739208802178717238e0_f64 * t105232 + 0.11514538467937585055e0_f64 * t98921 - 0.11514538467937585055e0_f64 * t98923 + 2.0_f64 * t855 * t2718 * t1911 * t21033 + 24.0_f64 * t855 * t40890 * t1911 * t21049 + 6.0_f64 * t855 * t2718 * t28431 * t1527;
    t105726
}
