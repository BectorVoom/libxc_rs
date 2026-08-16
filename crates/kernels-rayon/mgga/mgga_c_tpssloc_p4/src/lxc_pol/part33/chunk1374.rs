//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1374/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1374(t5: f64, t106780: f64, t106819: f64, t106847: f64, t106874: f64, t112: f64, t5456: f64, t7450: f64, t28025: f64, t4028: f64, t7458: f64, t105213: f64, t106617: f64, t106728: f64, t106733: f64, t106736: f64, t106738: f64, t106741: f64, t106744: f64, t106747: f64, t106753: f64, t106756: f64, t113: f64, t1869: f64, t1976: f64, t20347: f64, t20702: f64, t22425: f64, t24999: f64, t510: f64, t5450: f64, t5460: f64, t5494: f64, t6517: f64, t652: f64, t7670: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t106877 = piecewise3(t8, 0.0_f64, t106780 + t106819 + t106847 + t106874);
    let t106878 = t106877 * t112;
    let t106881 = t7450 * t5456;
    let t106889 = 6.0_f64 * t4028 * t28025;
    let t106891 = 6.0_f64 * t7458 * t28025;
    let t106892 = -2.0_f64 * t652 * t1976 * t20347 + t105213 - t113 * (t106617 + t106728) - t106733 - t106736 - t106738 + t106741 - t106744 - t106747 - 12.0_f64 * t24999 * t5460 - 6.0_f64 * t6517 * t20702 + t106753 + t106756 - t106878 * t510 - t1869 * t22425 - 6.0_f64 * t106881 * t510 - 3.0_f64 * t5450 * t7670 - 6.0_f64 * t24999 * t5494 - t106889 - t106891;
    (t106878, t106881, t106892)
}
