//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2697/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2697<F: Float>(t16398: F, t19966: F, t5259: F, t53945: F, t119: F, t12419: F, t1315: F, t16148: F, t16233: F, t16305: F, t16314: F, t16401: F, t19873: F, t19876: F, t19979: F, t19984: F, t20468: F, t210: F, t3793: F, t3805: F, t39936: F, t39948: F, t39950: F, t40168: F, t5246: F, t5301: F, t53921: F, t53927: F, t53929: F, t53946: F, t53965: F, t53973: F, t54013: F, t54014: F, t54258: F, t54614: F, t56275: F) -> F {
    let t56693 = t16398 * t19966;
    let t56710 = t53945 * t5259;
    let t56729 = -F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t53921 + F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t53927 + F::cast_from(7.0_f64) / F::cast_from(6.0_f64) * t53929 + t39936 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56693 - t19876 * t16314 / F::cast_from(96.0_f64) - t5246 * t16305 * t54258 * t20468 / F::cast_from(64.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53946 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t39948 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t39950 + t16401 * t19873 / F::cast_from(256.0_f64) - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t54614 * t40168 * t5301 * t16148 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t56710 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t5246 * t12419 * t19979 * t3793 - t5246 * t3805 * t19984 * t3793 / F::cast_from(384.0_f64) - t16233 * t54013 * t54014 * t53973 / F::cast_from(128.0_f64) - t1315 * t210 * t119 * t56275 / F::cast_from(48.0_f64) + F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t53965;
    t56729
}
