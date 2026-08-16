//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 748/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk748(t1388: f64, t1390: f64, t1297: f64, t1307: f64, t193: f64, t2408: f64, t2417: f64, t3683: f64, t3686: f64, t3688: f64, t3690: f64, t3693: f64, t3695: f64, t3697: f64, t3698: f64, t3701: f64, t3719: f64, t3813: f64, t3914: f64, t3918: f64, t533: f64) -> (f64, f64) {
    let t3919 = t1388 * t1390;
    let t3923 = t1390 * t193 * t3914 * t533 - t193 * t3698 * t3701 * t533 + 3.0_f64 * t1297 * t193 * t3719 + 6.0_f64 * t1307 * t3918 * t3919 + t2408 + t2417 + t3683 + t3686 + t3688 - t3690 - t3693 - t3695 + t3697 + t3813;
    (t3919, t3923)
}
