//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2294/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2294(t12030: f64, t12444: f64, t1375: f64, t22630: f64, t26226: f64, t26482: f64, t3752: f64, t3758: f64, t3887: f64, t3911: f64, t5321: f64, t568: f64, t7722: f64, t7729: f64, t7749: f64, t81264: f64, t90659: f64, t90663: f64, t90665: f64, t90670: f64) -> f64 {
    let t90677 = -6.0_f64 * t5321 * t22630 + 0.52089578783527170488e-1_f64 * t81264 + 2.0_f64 * t12030 * t7729 + 4.0_f64 * t12444 * t7729 - 0.63969658155208805863e-1_f64 * t90659 - 0.82246703342411321824e-2_f64 * t90663 - 12.0_f64 * t90665 * t26226 + 4.0_f64 * t3758 * t26482 + t90670 + 2.0_f64 * t1375 * t3887 * t7749 * t3911 + t3752 * t7722 * t568;
    t90677
}
