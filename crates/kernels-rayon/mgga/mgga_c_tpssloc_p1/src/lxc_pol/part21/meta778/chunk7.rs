//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2697/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2697(t16398: f64, t19966: f64, t5259: f64, t53945: f64, t119: f64, t12419: f64, t1315: f64, t16148: f64, t16233: f64, t16305: f64, t16314: f64, t16401: f64, t19873: f64, t19876: f64, t19979: f64, t19984: f64, t20468: f64, t210: f64, t3793: f64, t3805: f64, t39936: f64, t39948: f64, t39950: f64, t40168: f64, t5246: f64, t5301: f64, t53921: f64, t53927: f64, t53929: f64, t53946: f64, t53965: f64, t53973: f64, t54013: f64, t54014: f64, t54258: f64, t54614: f64, t56275: f64) -> f64 {
    let t56693 = t16398 * t19966;
    let t56710 = t53945 * t5259;
    let t56729 = -7.0_f64 / 12.0_f64 * t53921 + 35.0_f64 / 18.0_f64 * t53927 + 7.0_f64 / 6.0_f64 * t53929 + t39936 - 7.0_f64 / 1152.0_f64 * t56693 - t19876 * t16314 / 96.0_f64 - t5246 * t16305 * t54258 * t20468 / 64.0_f64 - 7.0_f64 / 288.0_f64 * t53946 - 119.0_f64 / 6912.0_f64 * t39948 - 119.0_f64 / 13824.0_f64 * t39950 + t16401 * t19873 / 256.0_f64 - 5.0_f64 / 32.0_f64 * t54614 * t40168 * t5301 * t16148 - 7.0_f64 / 288.0_f64 * t56710 + 5.0_f64 / 384.0_f64 * t5246 * t12419 * t19979 * t3793 - t5246 * t3805 * t19984 * t3793 / 384.0_f64 - t16233 * t54013 * t54014 * t53973 / 128.0_f64 - t1315 * t210 * t119 * t56275 / 48.0_f64 + 35.0_f64 / 288.0_f64 * t53965;
    t56729
}
