//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1349/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1349(t20447: f64, t219: f64, t10833: f64, t10894: f64, t10895: f64, t1378: f64, t1395: f64, t1396: f64, t17993: f64, t18006: f64, t18009: f64, t18021: f64, t1805: f64, t18750: f64, t18753: f64, t18770: f64, t18775: f64, t18779: f64, t19734: f64, t19736: f64, t20449: f64, t20471: f64, t20492: f64, t20498: f64, t2162: f64, t2408: f64, t2425: f64, t2426: f64, t3721: f64, t3722: f64, t5571: f64, t5572: f64, t5831: f64, t5834: f64, t5838: f64, t5846: f64, t62731: f64, t6337: f64, t64135: f64, t66328: f64, t819: f64) -> f64 {
    let t66525 = t20447 * t219;
    let t66546 = 2.0_f64 * t17993 * t20492 - 2.0_f64 * t18753 * t3722 - t62731 * t1396 - 4.0_f64 * t18006 * t18770 * t1378 * t18009 - t5834 * t10895 - 6.0_f64 * t5834 * t10833 - 2.0_f64 * t5571 * t18021 * t66328 * t2162 + 2.0_f64 * t19736 * t18779 + 2.0_f64 * t5571 * t5572 * t18750 * t1395 - t20449 * t2426 + 4.0_f64 * t64135 * t5838 + 4.0_f64 * t5571 * t5572 * t5831 * t3721 - 2.0_f64 * t66525 * t819 + 4.0_f64 * t17993 * t20498 + 2.0_f64 * t20449 * t2408 + 4.0_f64 * t17993 * t20471 + 2.0_f64 * t5571 * t5572 * t1805 * t10894 + 2.0_f64 * t5571 * t5572 * t6337 * t2425 - 2.0_f64 * t19734 * t5846 + 4.0_f64 * t19736 * t18775;
    t66546
}
