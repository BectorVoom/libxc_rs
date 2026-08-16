//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2150/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2150(t87729: f64, t25325: f64, t6547: f64, t13390: f64, t23016: f64, t25255: f64, t25262: f64, t25295: f64, t2679: f64, t2684: f64, t4162: f64, t4166: f64, t6660: f64, t808: f64, t812: f64, t82028: f64, t82032: f64, t82047: f64, t87699: f64, t87705: f64, t87708: f64, t87710: f64, t87714: f64, t87718: f64, t87726: f64) -> f64 {
    let t87730 = 0.82246703342411321824e-2_f64 * t87729;
    let t87733 = t6547 * t25325;
    let t87734 = 0.38381794893125283518e-1_f64 * t87733;
    let t87735 = -t4166 * t23016 + 0.41123351671205660912e-2_f64 * t82028 + 0.9869604401089358619e-1_f64 * t87699 + 2.0_f64 * t808 * t25295 + 0.3289868133696452873e-1_f64 * t87705 - 0.52089578783527170488e-1_f64 * t82032 - t87708 + t87710 - 0.49348022005446793096e-1_f64 * t87714 - t82047 - t812 * t25255 * t2679 - 0.52089578783527170489e-1_f64 * t87718 - t812 * t25255 * t2684 + 2.0_f64 * t4162 * t6660 - 0.16449340668482264365e-1_f64 * t87726 + t87730 - 2.0_f64 * t13390 * t25262 - t87734;
    t87735
}
