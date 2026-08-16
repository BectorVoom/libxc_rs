//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1189/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1189(t5: f64, t31688: f64, t8515: f64, t1862: f64, t79: f64, t641: f64, t8513: f64, t31019: f64, t31672: f64, t31675: f64, t31677: f64, t31681: f64, t31684: f64, t8512: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t31690 = 5.0_f64 / 27.0_f64 * t31688 * t8515;
    let t31691 = t79 * t1862;
    let t31693 = t8513 * t31691 * t641;
    let t31699 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t31672 * t8515 + 5.0_f64 / 12.0_f64 * t31675 * t31677 + 5.0_f64 / 18.0_f64 * t31681 * t31684 + t31690 - 5.0_f64 / 36.0_f64 * t8512 * t31693 - 5.0_f64 / 72.0_f64 * t8512 * t31019);
    (t31690, t31691, t31693, t31699)
}
