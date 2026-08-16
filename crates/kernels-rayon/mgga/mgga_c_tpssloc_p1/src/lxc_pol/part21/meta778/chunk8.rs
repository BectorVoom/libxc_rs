//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2698/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2698(t19844: f64, t3726: f64, t1831: f64, t53906: f64, t12419: f64, t12420: f64, t12429: f64, t16048: f64, t16224: f64, t16233: f64, t16305: f64, t16312: f64, t16333: f64, t16401: f64, t19871: f64, t19894: f64, t19945: f64, t19956: f64, t19979: f64, t19984: f64, t20473: f64, t3793: f64, t3803: f64, t3805: f64, t3851: f64, t5240: f64, t5246: f64, t5248: f64, t5287: f64, t5308: f64, t53984: f64, t53997: f64, t54003: f64, t54034: f64, t54043: f64) -> f64 {
    let t56738 = t3726 * t19844;
    let t56776 = t53906 * t1831;
    let t56778 = -35.0_f64 / 54.0_f64 * t53984 - 5.0_f64 / 768.0_f64 * t3803 * t12419 * t19956 * t12420 + 119.0_f64 / 864.0_f64 * t53997 - 7.0_f64 / 24.0_f64 * t54003 + 7.0_f64 / 72.0_f64 * t56738 - t5246 * t16305 * t20473 * t16312 / 192.0_f64 - t3803 * t5248 * t19871 * t3851 / 3072.0_f64 - 5.0_f64 / 768.0_f64 * t3803 * t12419 * t19979 * t3851 + t3803 * t3805 * t19984 * t3851 / 768.0_f64 - t16233 * t5248 * t19956 * t16048 / 512.0_f64 + t5246 * t5248 * t19956 * t3793 / 512.0_f64 + t16401 * t19945 / 384.0_f64 + 7.0_f64 / 2304.0_f64 * t54034 - 7.0_f64 / 1152.0_f64 * t54043 - 5.0_f64 / 192.0_f64 * t12429 * t19894 - 5.0_f64 / 192.0_f64 * t3803 * t16224 * t5287 * t5308 - t5240 * t16333 / 384.0_f64 + 7.0_f64 / 288.0_f64 * t56776;
    t56778
}
