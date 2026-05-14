//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 939/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk939<F: Float>(t12929: F, t13263: F, t19100: F, t19104: F, t19106: F, t19199: F, t25627: F, t25629: F, t25632: F, t25634: F, t25652: F, t1175: F, t13244: F, t13247: F, t1355: F, t19182: F, t19185: F, t2083: F, t25568: F, t25573: F, t25582: F, t25620: F, t25623: F, t306: F, t3599: F, t3602: F, t5662: F, t5684: F, t5687: F, t7757: F, t7764: F) -> (F,) {
    let t25653 = 0.14865e-1 * t25627 - 0.1982e-1 * t25629 - 0.991e-2 * t25632 + 0.1982e-1 * t25634 - t13263 - 0.18344444444444444444e-2 * t12929 - 0.36688888888888888888e-2 * t19100 + t19199 - 0.55033333333333333332e-2 * t19104 + 0.55033333333333333332e-2 * t19106 + t25652;
    let t25656 = 3.0 / 16.0 * t13244 * t25568 - t13247 * t7757 / 8.0 - t3599 * t25573 / 4.0 - t19182 * t5662 / 4.0 + t19185 * t2083 / 2.0 + t5687 * t5684 / 2.0 - t3599 * t25582 / 8.0 + t3602 * t7764 / 4.0 + t1355 * t25620 / 4.0 + t25623 * t1175 / 4.0 + t306 * t25653 / 2.0;
    (t25656,)
}
