//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2659/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2659<F: Float>(t1824: F, t3791: F, t12168: F, t12369: F, t1352: F, t16224: F, t16305: F, t16364: F, t3803: F, t3805: F, t3851: F, t40089: F, t40114: F, t40116: F, t40124: F, t40126: F, t40128: F, t40131: F, t40139: F, t40145: F, t5246: F, t5248: F, t5249: F, t53958: F, t54068: F) -> (F, F) {
    let t54258 = t1824 * t3791;
    let t54277 = -F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t40089 + t3803 * t3805 * t16364 * t3851 / F::cast_from(256.0_f64) - t3803 * t5248 * t53958 * t1352 / F::cast_from(1024.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t40114 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t40116 - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t5246 * t16305 * t54258 * t12369 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t5246 * t16224 * t54068 * t12369 + F::cast_from(595.0_f64) / F::cast_from(3456.0_f64) * t40124 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t40126 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t40128 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t40131 - t3803 * t5248 * t5249 * t12168 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t40139 - F::cast_from(595.0_f64) / F::cast_from(3456.0_f64) * t40145;
    (t54258, t54277)
}
