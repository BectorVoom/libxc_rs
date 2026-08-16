//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1646/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1646(t1338: f64, t7191: f64, t1352: f64, t24063: f64, t553: f64, t2085: f64, t3787: f64, t3793: f64, t3856: f64, t7208: f64, t1336: f64, t22735: f64, t22743: f64, t22745: f64, t22749: f64, t22752: f64, t22884: f64, t22888: f64, t22895: f64, t22900: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24116 = t1338 * t7191;
    let t24117 = t24116 * t1352;
    let t24121 = t553 * t24063;
    let t24127 = t3787 * t2085;
    let t24128 = t24127 * t3793;
    let t24131 = t7208 * t3856;
    let t24137 = -2.0_f64 * t1336 * t24117 + 0.6579736267392905746e-1_f64 * t22735 + t544 * t24121 - 0.16449340668482264365e-1_f64 * t22743 + 0.76763589786250567036e-1_f64 * t22745 + 0.9869604401089358619e-1_f64 * t22749 + 0.15352717957250113407e0_f64 * t22752 + 2.0_f64 * t1336 * t24128 - t1336 * t24131 - 0.6579736267392905746e-1_f64 * t22884 - 0.3289868133696452873e-1_f64 * t22888 + 0.3289868133696452873e-1_f64 * t22895 + 0.3289868133696452873e-1_f64 * t22900;
    (t24116, t24117, t24121, t24128, t24131, t24137)
}
