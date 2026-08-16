//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1303/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1303(t1888: f64, t32862: f64, t82159: f64, t112667: f64, t112673: f64, t23270: f64, t25170: f64, t112678: f64, t112680: f64, t112686: f64, t112702: f64, t30713: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t118498 = 0.3289868133696452873e-1_f64 * t1888 * t82159 * t32862;
    let t118499 = 0.38381794893125283518e-1_f64 * t112667;
    let t118500 = 0.38381794893125283518e-1_f64 * t112673;
    let t118503 = 0.9869604401089358619e-1_f64 * t1888 * t23270 * t25170;
    let t118506 = 0.82246703342411321825e-2_f64 * t112678;
    let t118518 = 0.76763589786250567036e-1_f64 * t112680;
    let t118523 = 0.76763589786250567036e-1_f64 * t112686;
    let t118526 = 0.16449340668482264365e-1_f64 * t112702;
    let t118532 = t4166 * t30713;
    (t118498, t118499, t118500, t118503, t118506, t118518, t118523, t118526, t118532)
}
