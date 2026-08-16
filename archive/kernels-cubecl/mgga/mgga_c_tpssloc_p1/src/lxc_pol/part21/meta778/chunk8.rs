//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2698/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2698<F: Float>(t19844: F, t3726: F, t1831: F, t53906: F, t12419: F, t12420: F, t12429: F, t16048: F, t16224: F, t16233: F, t16305: F, t16312: F, t16333: F, t16401: F, t19871: F, t19894: F, t19945: F, t19956: F, t19979: F, t19984: F, t20473: F, t3793: F, t3803: F, t3805: F, t3851: F, t5240: F, t5246: F, t5248: F, t5287: F, t5308: F, t53984: F, t53997: F, t54003: F, t54034: F, t54043: F) -> F {
    let t56738 = t3726 * t19844;
    let t56776 = t53906 * t1831;
    let t56778 = -F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t53984 - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3803 * t12419 * t19956 * t12420 + F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t53997 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t54003 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t56738 - t5246 * t16305 * t20473 * t16312 / F::cast_from(192.0_f64) - t3803 * t5248 * t19871 * t3851 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t3803 * t12419 * t19979 * t3851 + t3803 * t3805 * t19984 * t3851 / F::cast_from(768.0_f64) - t16233 * t5248 * t19956 * t16048 / F::cast_from(512.0_f64) + t5246 * t5248 * t19956 * t3793 / F::cast_from(512.0_f64) + t16401 * t19945 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t54034 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t54043 - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t12429 * t19894 - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t3803 * t16224 * t5287 * t5308 - t5240 * t16333 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t56776;
    t56778
}
