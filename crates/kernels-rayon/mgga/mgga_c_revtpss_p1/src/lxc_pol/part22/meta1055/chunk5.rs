//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3735/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3735(t20900: f64, t73: f64, t12866: f64, t17654: f64, t17693: f64, t17694: f64, t17695: f64, t17756: f64, t17794: f64, t20767: f64, t20932: f64, t21063: f64, t3362: f64, t3367: f64, t3663: f64, t3718: f64, t372: f64, t3720: f64, t44230: f64, t44458: f64, t44517: f64, t5277: f64, t5333: f64, t5352: f64, t56977: f64, t57689: f64, t58909: f64, t6640: f64, t70910: f64, t70914: f64, t70917: f64, t70933: f64, t70942: f64) -> (f64, f64) {
    let t70944 = t20900 * t73;
    let t70953 = -0.16937883700965822013e-2_f64 * t57689 + 0.95275595817932748828e-3_f64 * t17693 * t17694 * t70910 - 0.57165357490759649296e-3_f64 * t70914 - 0.22866142996303859718e-2_f64 * t70917 * t17756 - 0.11433071498151929859e-2_f64 * t56977 * t20767 - 0.95275595817932748826e-3_f64 * t12866 * t372 * t17794 * t3362 * t17695 + 0.11433071498151929859e-2_f64 * t12866 * t372 * t5277 * t3367 * t17695 - 0.22866142996303859718e-2_f64 * t17654 * t58909 * t44458 * t70933 - 0.57165357490759649296e-3_f64 * t44517 * t58909 * t5333 * t20932 - 11.0_f64 / 486.0_f64 * t70942 - 0.42874018118069736972e-3_f64 * t3718 * t3720 * t70944 * t5352 + 0.22866142996303859718e-2_f64 * t21063 * t3663 - 0.28582678745379824648e-3_f64 * t44230 * t6640;
    (t70944, t70953)
}
