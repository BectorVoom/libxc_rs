//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3141/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3141(t1214: f64, t24616: f64, t5245: f64, t6573: f64, t21143: f64, t5378: f64, t21192: f64, t5391: f64, t17344: f64, t17396: f64, t17412: f64, t1808: f64, t20986: f64, t21037: f64, t21042: f64, t247: f64, t3719: f64, t5381: f64, t5384: f64, t5397: f64, t57520: f64, t6673: f64, t70647: f64, t71590: f64, t82207: f64) -> (f64, f64, f64) {
    let t82514 = t24616 * t1214;
    let t82525 = t6573 * t5245;
    let t82534 = t21143 * t5378;
    let t82536 = t5391 * t21192;
    let t82542 = 0.34299214494455789577e-2_f64 * t17396 * t21042 + 0.51448821741683684368e-2_f64 * t57520 * t247 * t3719 * t82514 + 0.12862205435420921092e-2_f64 * t5384 * t247 * t3719 * t82207 - 0.3811023832717309953e-2_f64 * t17412 * t6673 - 0.38586616306262763276e-2_f64 * t17344 * t247 * t3719 * t82525 - 0.42874018118069736972e-3_f64 * t71590 * t1808 - 0.42874018118069736972e-3_f64 * t21143 * t5397 - 0.28582678745379824648e-3_f64 * t82534 + 0.30488190661738479624e-2_f64 * t82536 - 0.91464571985215438872e-2_f64 * t70647 * t21037 - 0.25724410870841842183e-2_f64 * t5381 * t20986;
    (t82514, t82525, t82542)
}
