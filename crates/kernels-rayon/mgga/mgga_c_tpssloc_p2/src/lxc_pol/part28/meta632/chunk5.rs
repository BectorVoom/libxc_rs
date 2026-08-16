//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1994/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1994(t87898: f64, t87901: f64, t87910: f64, t87915: f64, t87927: f64, t87931: f64, t10109: f64, t7106: f64, t13058: f64, t13461: f64, t1528: f64, t24305: f64, t25168: f64, t26728: f64, t2718: f64, t4272: f64, t4300: f64, t4301: f64, t7087: f64, t82294: f64, t82296: f64, t85079: f64, t855: f64, t87924: f64) -> (f64, f64, f64, f64, f64) {
    let t92954 = 0.52089578783527170489e-1_f64 * t87898;
    let t92955 = 0.3289868133696452873e-1_f64 * t87901;
    let t92960 = 0.16449340668482264365e-1_f64 * t87910;
    let t92961 = 0.16449340668482264365e-1_f64 * t87915;
    let t92966 = 0.9869604401089358619e-1_f64 * t87927;
    let t92976 = 0.15352717957250113407e0_f64 * t87931;
    let t92981 = t10109 * t7106;
    let t92985 = 0.9869604401089358619e-1_f64 * t87924 - t92966 - t85079 * t1528 - 0.20835831513410868196e0_f64 * t82294 - 0.23029076935875170111e0_f64 * t82296 - 2.0_f64 * t24305 * t4301 - 6.0_f64 * t25168 * t26728 * t13058 - t7087 * t13461 - t92976 + 4.0_f64 * t855 * t2718 * t7106 * t4300 - 12.0_f64 * t25168 * t92981 * t4272;
    (t92954, t92955, t92960, t92961, t92985)
}
