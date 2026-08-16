//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1224/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1224(t5: f64, t1409: f64, t31682: f64, t8308: f64, t1433: f64, t31691: f64, t8513: f64, t12571: f64, t8662: f64, t7973: f64, t8301: f64, t2240: f64, t31860: f64, t31864: f64, t33115: f64, t33564: f64, t8515: f64, t8663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33567 = t31682 * t1409;
    let t33568 = t8308 * t33567;
    let t33572 = t8513 * t31691 * t1433;
    let t33669 = t12571 * t8662;
    let t33676 = t8301 * t7973;
    let t33677 = t2240 * t33676;
    let t33685 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t33669 * t8515 - 5.0_f64 / 24.0_f64 * t31860 * t33564 - 5.0_f64 / 36.0_f64 * t31864 * t33568 + 5.0_f64 / 144.0_f64 * t33677 * t8515 + 5.0_f64 / 72.0_f64 * t8663 * t33572 + 5.0_f64 / 144.0_f64 * t8663 * t33115);
    (t33568, t33572, t33669, t33676, t33677, t33685)
}
