//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1374/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1374<F: Float>(t5: F, t106780: F, t106819: F, t106847: F, t106874: F, t112: F, t5456: F, t7450: F, t28025: F, t4028: F, t7458: F, t105213: F, t106617: F, t106728: F, t106733: F, t106736: F, t106738: F, t106741: F, t106744: F, t106747: F, t106753: F, t106756: F, t113: F, t1869: F, t1976: F, t20347: F, t20702: F, t22425: F, t24999: F, t510: F, t5450: F, t5460: F, t5494: F, t6517: F, t652: F, t7670: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t106877 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t106780 + t106819 + t106847 + t106874);
    let t106878 = t106877 * t112;
    let t106881 = t7450 * t5456;
    let t106889 = F::cast_from(6.0_f64) * t4028 * t28025;
    let t106891 = F::cast_from(6.0_f64) * t7458 * t28025;
    let t106892 = -F::cast_from(2.0_f64) * t652 * t1976 * t20347 + t105213 - t113 * (t106617 + t106728) - t106733 - t106736 - t106738 + t106741 - t106744 - t106747 - F::cast_from(12.0_f64) * t24999 * t5460 - F::cast_from(6.0_f64) * t6517 * t20702 + t106753 + t106756 - t106878 * t510 - t1869 * t22425 - F::cast_from(6.0_f64) * t106881 * t510 - F::cast_from(3.0_f64) * t5450 * t7670 - F::cast_from(6.0_f64) * t24999 * t5494 - t106889 - t106891;
    (t106878, t106881, t106892)
}
