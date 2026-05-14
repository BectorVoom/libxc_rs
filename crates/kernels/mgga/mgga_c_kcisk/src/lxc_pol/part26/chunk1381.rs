//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1381/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1381<F: Float>(t34941: F, t9532: F, t115454: F, t115463: F, t115468: F, t115489: F, t115493: F, t119124: F, t119128: F, t1299: F, t20: F, t2734: F, t2740: F, t32417: F, t32474: F, t33807: F, t34969: F, t35025: F, t8306: F, t9523: F, t9859: F) -> (F,) {
    let t120330 = t34941 * t9532;
    let t120341 = -0.52083333333333333333e-2 * t35025 * t9523 * t2740 - 0.10416666666666666667e-1 * t33807 * t9859 * t2740 + 0.13888888888888888889e-1 * t2734 * t8306 * t1299 * t20 * t2740 - 0.17361111111111111111e-2 * t120330 + 0.23148148148148148148e-2 * t115454 + 0.10317654320987654321e-2 * t119124 - 0.92592592592592592592e-2 * t115463 + 0.20104166666666666667e-2 * t32474 * t34969 + 0.20104166666666666667e-2 * t32417 * t34969 + 0.92592592592592592592e-2 * t115468 + 0.51588271604938271604e-3 * t119128 + t115489 - t115493;
    (t120341,)
}
