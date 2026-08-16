//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3120/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3120<F: Float>(t11243: F, t1802: F, t1244: F, t13036: F, t12881: F, t5391: F, t1222: F, t16720: F, t17471: F, t11231: F, t12784: F, t12787: F, t12812: F, t12855: F, t12922: F, t13065: F, t16719: F, t16756: F, t17351: F, t17355: F, t17505: F, t17600: F, t17633: F, t17640: F, t17649: F, t17654: F, t17703: F, t17742: F, t21028: F, t21119: F, t3718: F, t3720: F, t44508: F, t44769: F, t5312: F, t5331: F, t5333: F, t5340: F, t5348: F, t56205: F, t57005: F, t57373: F, t57378: F, t57382: F, t57386: F, t57394: F) -> (F, F) {
    let t57403 = t1802 * t11243;
    let t57405 = t13036 * t1244 * t57403;
    let t57421 = t5391 * t12881;
    let t57422 = F::cast_from(0.5081365110289746604e-3_f64) * t57421;
    let t57428 = t1222 * t17471 * t16720;
    let t57433 = -F::cast_from(0.64311027177104605458e-3_f64) * t5331 * t3720 * t57373 * t5333 + F::cast_from(0.85748036236139473944e-3_f64) * t57378 * t17355 + F::cast_from(0.64311027177104605458e-3_f64) * t57382 * t12812 - F::cast_from(0.42874018118069736972e-3_f64) * t57386 - F::cast_from(0.64311027177104605458e-3_f64) * t44769 * t5348 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t17633 * t17742 - F::cast_from(0.85748036236139473944e-3_f64) * t17654 * t17649 * t57394 * t21119 + F::cast_from(0.42874018118069736972e-3_f64) * t17351 * t17649 * t57394 * t21028 - F::cast_from(0.11433071498151929859e-2_f64) * t57405 * t13065 + F::cast_from(0.42874018118069736972e-2_f64) * t57005 * t12787 * t16719 * t11231 - F::cast_from(0.42874018118069736972e-3_f64) * t12784 * t17640 - F::cast_from(0.45732285992607719436e-2_f64) * t17505 * t12922 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t17600 * t21119 - F::cast_from(0.42874018118069736972e-3_f64) * t44508 - t57422 + F::cast_from(0.12862205435420921092e-2_f64) * t5340 * t3720 * t16756 * t17703 + t57428 / F::cast_from(36.0_f64) + t1222 * t5312 * t56205 / F::cast_from(216.0_f64);
    (t57403, t57433)
}
