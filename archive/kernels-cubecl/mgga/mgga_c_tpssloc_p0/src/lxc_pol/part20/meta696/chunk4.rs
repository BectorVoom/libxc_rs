//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2658/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658<F: Float>(t40138: F, t5303: F, t12283: F, t16366: F, t16308: F, t1352: F, t16153: F, t16224: F, t16242: F, t16311: F, t3803: F, t3805: F, t3856: F, t40052: F, t40054: F, t40060: F, t40065: F, t40079: F, t40081: F, t40083: F, t40178: F, t5246: F, t5248: F, t5249: F, t54013: F, t54015: F) -> F {
    let t54220 = t40138 * t5303;
    let t54222 = t12283 * t16366;
    let t54237 = t12283 * t16308;
    let t54245 = -t3803 * t5248 * t16242 * t3856 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t54220 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t54222 + t3803 * t3805 * t5249 * t40178 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t16153 * t1352 + F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t40052 + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t5246 * t54013 * t16311 * t54015 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t54237 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t40054 + F::cast_from(595.0_f64) / F::cast_from(864.0_f64) * t40060 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t40065 + F::cast_from(595.0_f64) / F::cast_from(1152.0_f64) * t40079 + F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t40081 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t40083;
    t54245
}
