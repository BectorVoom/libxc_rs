//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1355/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1355(t21854: f64, t508: f64, t2157: f64, t6337: f64, t1378: f64, t14297: f64, t14372: f64, t17993: f64, t18000: f64, t18006: f64, t1805: f64, t18753: f64, t18770: f64, t19736: f64, t19748: f64, t19767: f64, t19769: f64, t19781: f64, t20446: f64, t20475: f64, t20479: f64, t20482: f64, t20498: f64, t21299: f64, t21608: f64, t21650: f64, t226: f64, t3664: f64, t4758: f64, t4783: f64, t4799: f64, t4800: f64, t52460: f64, t5571: f64, t5572: f64, t5577: f64, t5831: f64, t5834: f64, t5846: f64, t64060: f64, t66362: f64, t66480: f64, t70070: f64, t818: f64) -> (f64, f64) {
    let t71884 = t508 * t21854;
    let t71935 = t2157 * t6337;
    let t71970 = -t21299 * t5846 + 2.0_f64 * t5834 * t14372 + 2.0_f64 * t5571 * t5572 * t5831 * t4799 + 2.0_f64 * t5571 * t5572 * t21608 * t818 + 4.0_f64 * t19736 * t20475 - 4.0_f64 * t18006 * t66480 * t19781 + 4.0_f64 * t19736 * t20498 - 6.0_f64 * t5571 * t18000 * t5831 * t4783 - 4.0_f64 * t19767 * t71935 * t19769 - 2.0_f64 * t19767 * t20482 * t52460 - 4.0_f64 * t18006 * t66362 * t19748 - 2.0_f64 * t18006 * t18770 * t70070 + t5571 * t5577 * t5831 * t4758 * t226 + t5571 * t5577 * t1805 * t14297 * t226 - t18753 * t4800 + 2.0_f64 * t5571 * t5577 * t20446 * t1378 * t226 + 2.0_f64 * t5571 * t5577 * t6337 * t3664 * t226 + t17993 * t21650 - 4.0_f64 * t64060 * t20479;
    (t71884, t71970)
}
