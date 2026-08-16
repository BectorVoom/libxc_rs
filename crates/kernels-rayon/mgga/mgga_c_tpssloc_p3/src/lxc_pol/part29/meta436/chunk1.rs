//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1737/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1737(t1332: f64, t1336: f64, t22693: f64, t22697: f64, t22701: f64, t22707: f64, t22710: f64, t22718: f64, t22721: f64, t22726: f64, t22728: f64, t22731: f64, t22735: f64, t3777: f64, t6988: f64, t6990: f64) -> f64 {
    let t22739 = -t22693 - 0.16449340668482264365e-1_f64 * t22697 - 0.82246703342411321825e-2_f64 * t22701 + 0.82246703342411321824e-2_f64 * t22707 + 2.0_f64 * t1336 * t22710 - 2.0_f64 * t3777 * t6988 + t22718 + 0.82246703342411321825e-2_f64 * t22721 + t22726 - 0.82246703342411321824e-2_f64 * t22728 - t22731 + 0.3289868133696452873e-1_f64 * t22735 + 2.0_f64 * t1332 * t6990;
    t22739
}
