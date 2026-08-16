//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3735/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3735<F: Float>(t20900: F, t73: F, t12866: F, t17654: F, t17693: F, t17694: F, t17695: F, t17756: F, t17794: F, t20767: F, t20932: F, t21063: F, t3362: F, t3367: F, t3663: F, t3718: F, t372: F, t3720: F, t44230: F, t44458: F, t44517: F, t5277: F, t5333: F, t5352: F, t56977: F, t57689: F, t58909: F, t6640: F, t70910: F, t70914: F, t70917: F, t70933: F, t70942: F) -> (F, F) {
    let t70944 = t20900 * t73;
    let t70953 = -F::cast_from(0.16937883700965822013e-2_f64) * t57689 + F::cast_from(0.95275595817932748828e-3_f64) * t17693 * t17694 * t70910 - F::cast_from(0.57165357490759649296e-3_f64) * t70914 - F::cast_from(0.22866142996303859718e-2_f64) * t70917 * t17756 - F::cast_from(0.11433071498151929859e-2_f64) * t56977 * t20767 - F::cast_from(0.95275595817932748826e-3_f64) * t12866 * t372 * t17794 * t3362 * t17695 + F::cast_from(0.11433071498151929859e-2_f64) * t12866 * t372 * t5277 * t3367 * t17695 - F::cast_from(0.22866142996303859718e-2_f64) * t17654 * t58909 * t44458 * t70933 - F::cast_from(0.57165357490759649296e-3_f64) * t44517 * t58909 * t5333 * t20932 - F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t70942 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t3720 * t70944 * t5352 + F::cast_from(0.22866142996303859718e-2_f64) * t21063 * t3663 - F::cast_from(0.28582678745379824648e-3_f64) * t44230 * t6640;
    (t70944, t70953)
}
