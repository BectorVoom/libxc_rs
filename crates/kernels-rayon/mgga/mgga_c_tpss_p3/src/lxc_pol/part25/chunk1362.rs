//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1362/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1362(t21609: f64, t219: f64, t1395: f64, t1396: f64, t14363: f64, t14367: f64, t14424: f64, t1707: f64, t1708: f64, t17993: f64, t1809: f64, t18753: f64, t19734: f64, t19736: f64, t20446: f64, t20449: f64, t20471: f64, t20483: f64, t20488: f64, t20492: f64, t20506: f64, t21631: f64, t21640: f64, t21656: f64, t228: f64, t3722: f64, t4784: f64, t5568: f64, t5571: f64, t5572: f64, t5834: f64, t5838: f64, t6135: f64, t6351: f64, t66525: f64, t69912: f64, t70039: f64, t70189: f64, t72079: f64, t819: f64) -> f64 {
    let t72153 = t21609 * t219;
    let t72170 = -t5834 * t14424 + 4.0_f64 * t5834 * t14367 - 2.0_f64 * t20449 * t3722 - 2.0_f64 * t17993 * t21640 - 2.0_f64 * t6135 * t20506 - t70189 * t1809 + 2.0_f64 * t69912 * t5838 - 6.0_f64 * t5834 * t14363 - t5568 * t21656 - 2.0_f64 * t66525 * t1396 + 2.0_f64 * t18753 * t4784 - 4.0_f64 * t70039 * t20483 + 4.0_f64 * t17993 * t21631 - t72153 * t819 - t1707 * t1708 * t228 * t72079 + 2.0_f64 * t19736 * t20488 + 2.0_f64 * t19736 * t20492 - 2.0_f64 * t19734 * t6351 + 4.0_f64 * t19736 * t20471 + 4.0_f64 * t5571 * t5572 * t20446 * t1395;
    t72170
}
