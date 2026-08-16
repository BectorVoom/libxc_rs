//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2696/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2696(t3791: f64, t40046: f64, t16398: f64, t20004: f64, t19945: f64, t120: f64, t1352: f64, t16018: f64, t16048: f64, t16233: f64, t16242: f64, t19631: f64, t19871: f64, t19989: f64, t3803: f64, t3805: f64, t5248: f64, t5249: f64, t53881: f64, t53883: f64, t53893: f64, t53895: f64, t53897: f64, t53901: f64, t53903: f64, t53907: f64, t53917: f64, t53919: f64, t54744: f64, t550: f64) -> (f64, f64) {
    let t56666 = t40046 * t3791;
    let t56685 = t16398 * t20004;
    let t56687 = t16398 * t19945;
    let t56689 = t3803 * t3805 * t16242 * t19989 / 192.0_f64 + t3803 * t3805 * t5249 * t550 * t16018 / 384.0_f64 + t3803 * t3805 * t120 * t19631 * t1352 / 384.0_f64 + t54744 * t5248 * t19871 * t56666 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t16233 * t5248 * t19871 * t16048 - 119.0_f64 / 864.0_f64 * t53881 + 7.0_f64 / 576.0_f64 * t53883 + 7.0_f64 / 576.0_f64 * t53893 + 7.0_f64 / 576.0_f64 * t53895 + 7.0_f64 / 288.0_f64 * t53897 + 595.0_f64 / 1296.0_f64 * t53901 - 35.0_f64 / 576.0_f64 * t53903 + 7.0_f64 / 288.0_f64 * t53907 - 119.0_f64 / 864.0_f64 * t53917 - 119.0_f64 / 864.0_f64 * t53919 + 7.0_f64 / 288.0_f64 * t56685 - 7.0_f64 / 576.0_f64 * t56687;
    (t56666, t56689)
}
