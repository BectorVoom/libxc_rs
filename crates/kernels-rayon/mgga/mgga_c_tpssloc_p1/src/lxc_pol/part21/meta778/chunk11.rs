//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2701/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2701(t19815: f64, t3802: f64, t20000: f64, t54566: f64, t16398: f64, t19873: f64, t16397: f64, t5234: f64, t5252: f64, t12429: f64, t16244: f64, t16265: f64, t16383: f64, t16394: f64, t16401: f64, t19871: f64, t19966: f64, t19986: f64, t19991: f64, t20004: f64, t3803: f64, t3805: f64, t3809: f64, t39993: f64, t5246: f64, t53958: f64, t54125: f64, t54131: f64, t54133: f64, t54135: f64, t54138: f64, t6394: f64) -> f64 {
    let t56878 = t19815 * t3802;
    let t56883 = t54566 * t20000;
    let t56885 = t16398 * t19873;
    let t56888 = t5234 * t16397 * t5252;
    let t56904 = 7.0_f64 / 1152.0_f64 * t54125 + 595.0_f64 / 864.0_f64 * t54131 - 35.0_f64 / 288.0_f64 * t54133 - 35.0_f64 / 288.0_f64 * t54135 - 35.0_f64 / 576.0_f64 * t54138 + t12429 * t19986 / 384.0_f64 + t3803 * t3805 * t53958 * t6394 / 384.0_f64 + t56878 * t3809 / 384.0_f64 + t16394 * t16244 / 192.0_f64 + 7.0_f64 / 384.0_f64 * t56883 - 7.0_f64 / 384.0_f64 * t56885 - 7.0_f64 / 576.0_f64 * t56888 + t12429 * t19991 / 192.0_f64 - t16401 * t20004 / 192.0_f64 - t5246 * t3805 * t19871 * t39993 / 384.0_f64 + t16401 * t19966 / 768.0_f64 + t16394 * t16383 / 384.0_f64 - t16394 * t16265 / 1536.0_f64;
    t56904
}
