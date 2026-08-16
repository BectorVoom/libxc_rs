//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 744/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk744(t3806: f64, t866: f64, t846: f64, t1424: f64, t2533: f64, t865: f64, t2531: f64, t2455: f64, t2537: f64, t3746: f64, t3751: f64, t3756: f64, t3760: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3807 = t3806 * t866;
    let t3809 = 1.0_f64 * t846 * t3807;
    let t3810 = t1424 * t2533;
    let t3811 = t3810 * t865;
    let t3813 = 0.16081979498692535067e2_f64 * t2531 * t3811;
    let t3819 = t2537 + 0.57077777777777777777e-2_f64 * t2455 + 0.57077777777777777777e-2_f64 * t3746 - 0.11415555555555555555e-1_f64 * t3751 + 0.34246666666666666666e-1_f64 * t3756 - 0.17123333333333333333e-1_f64 * t3760;
    (t3807, t3809, t3810, t3811, t3813, t3819)
}
