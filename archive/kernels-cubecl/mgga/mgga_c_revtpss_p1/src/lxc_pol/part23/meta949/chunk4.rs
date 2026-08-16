//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3141/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3141<F: Float>(t1214: F, t24616: F, t5245: F, t6573: F, t21143: F, t5378: F, t21192: F, t5391: F, t17344: F, t17396: F, t17412: F, t1808: F, t20986: F, t21037: F, t21042: F, t247: F, t3719: F, t5381: F, t5384: F, t5397: F, t57520: F, t6673: F, t70647: F, t71590: F, t82207: F) -> (F, F, F) {
    let t82514 = t24616 * t1214;
    let t82525 = t6573 * t5245;
    let t82534 = t21143 * t5378;
    let t82536 = t5391 * t21192;
    let t82542 = F::cast_from(0.34299214494455789577e-2_f64) * t17396 * t21042 + F::cast_from(0.51448821741683684368e-2_f64) * t57520 * t247 * t3719 * t82514 + F::cast_from(0.12862205435420921092e-2_f64) * t5384 * t247 * t3719 * t82207 - F::cast_from(0.3811023832717309953e-2_f64) * t17412 * t6673 - F::cast_from(0.38586616306262763276e-2_f64) * t17344 * t247 * t3719 * t82525 - F::cast_from(0.42874018118069736972e-3_f64) * t71590 * t1808 - F::cast_from(0.42874018118069736972e-3_f64) * t21143 * t5397 - F::cast_from(0.28582678745379824648e-3_f64) * t82534 + F::cast_from(0.30488190661738479624e-2_f64) * t82536 - F::cast_from(0.91464571985215438872e-2_f64) * t70647 * t21037 - F::cast_from(0.25724410870841842183e-2_f64) * t5381 * t20986;
    (t82514, t82525, t82542)
}
