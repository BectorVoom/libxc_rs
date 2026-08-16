//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3766/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3766<F: Float>(t17472: F, t5373: F, t1222: F, t17471: F, t20266: F, t17351: F, t20770: F, t56756: F, t1214: F, t12839: F, t12866: F, t17352: F, t17475: F, t17479: F, t17643: F, t21213: F, t3701: F, t372: F, t44510: F, t44769: F, t5312: F, t58909: F, t59320: F, t59336: F, t6690: F, t68285: F, t68290: F, t68340: F, t70932: F, t71452: F, t73: F) -> F {
    let t71880 = t5373 * t17472;
    let t71883 = t1222 * t17471 * t20266;
    let t71886 = t17351 * t56756 * t20770;
    let t71905 = -F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5373 * t17479 + t1222 * t5312 * t68285 / F::cast_from(216.0_f64) + t1222 * t5312 * t68290 / F::cast_from(36.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1222 * t17475 * t68340 - F::cast_from(4.0_f64) / F::cast_from(243.0_f64) * t71880 + t71883 / F::cast_from(324.0_f64) + F::cast_from(0.3811023832717309953e-3_f64) * t71886 - F::cast_from(0.6351706387862183255e-3_f64) * t59320 - F::cast_from(0.42874018118069736972e-3_f64) * t44769 * t6690 + F::cast_from(0.11433071498151929859e-2_f64) * t44510 * t58909 * t12839 * t70932 * t1214 + F::cast_from(0.11433071498151929859e-2_f64) * t12866 * t372 * t17352 * t73 * t17643 * t71452 + F::cast_from(11.0_f64) / F::cast_from(243.0_f64) * t21213 * t3701 + F::cast_from(0.10162730220579493208e-2_f64) * t59336;
    t71905
}
