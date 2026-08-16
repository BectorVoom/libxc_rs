//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3766/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3766(t17472: f64, t5373: f64, t1222: f64, t17471: f64, t20266: f64, t17351: f64, t20770: f64, t56756: f64, t1214: f64, t12839: f64, t12866: f64, t17352: f64, t17475: f64, t17479: f64, t17643: f64, t21213: f64, t3701: f64, t372: f64, t44510: f64, t44769: f64, t5312: f64, t58909: f64, t59320: f64, t59336: f64, t6690: f64, t68285: f64, t68290: f64, t68340: f64, t70932: f64, t71452: f64, t73: f64) -> f64 {
    let t71880 = t5373 * t17472;
    let t71883 = t1222 * t17471 * t20266;
    let t71886 = t17351 * t56756 * t20770;
    let t71905 = -4.0_f64 / 27.0_f64 * t5373 * t17479 + t1222 * t5312 * t68285 / 216.0_f64 + t1222 * t5312 * t68290 / 36.0_f64 - 7.0_f64 / 648.0_f64 * t1222 * t17475 * t68340 - 4.0_f64 / 243.0_f64 * t71880 + t71883 / 324.0_f64 + 0.3811023832717309953e-3_f64 * t71886 - 0.6351706387862183255e-3_f64 * t59320 - 0.42874018118069736972e-3_f64 * t44769 * t6690 + 0.11433071498151929859e-2_f64 * t44510 * t58909 * t12839 * t70932 * t1214 + 0.11433071498151929859e-2_f64 * t12866 * t372 * t17352 * t73 * t17643 * t71452 + 11.0_f64 / 243.0_f64 * t21213 * t3701 + 0.10162730220579493208e-2_f64 * t59336;
    t71905
}
