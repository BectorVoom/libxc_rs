//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1975/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1975(t3701: f64, t6995: f64, t1307: f64, t2018: f64, t7752: f64, t1458: f64, t576: f64, t2113: f64, t1390: f64, t22811: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31035 = t3701 * t6995;
    let t31299 = t2018 * t1307;
    let t33136 = t3701 * t7752;
    let t33185 = t576 * t1458;
    let t33690 = t2113 * t1458;
    let t34999 = t7752 * t1390;
    let t39041 = 1.0_f64 / t22811;
    let t39054 = t601 * t9238;
    (t31035, t31299, t33136, t33185, t33690, t34999, t39041, t39054)
}
