//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2681/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2681<F: Float>(t12178: F, t12255: F, t12303: F, t12371: F, t16305: F, t16311: F, t16312: F, t19735: F, t19876: F, t3803: F, t3805: F, t3807: F, t40168: F, t40285: F, t40293: F, t40295: F, t5246: F, t5301: F, t54258: F, t54585: F, t54591: F, t54607: F, t54609: F, t54612: F, t54614: F) -> F {
    let t54625 = F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t54585 - t5246 * t16305 * t19735 * t16312 / F::cast_from(64.0_f64) - t5246 * t16305 * t16311 * t54591 / F::cast_from(128.0_f64) + t3803 * t16305 * t54258 * t3807 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t40285 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t40293 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t40295 - t5246 * t3805 * t5301 * t12255 / F::cast_from(128.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t54607 - F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t54609 + t54612 - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t54614 * t40168 * t5301 * t12303 - t19876 * t12371 / F::cast_from(128.0_f64) + t3803 * t3805 * t5301 * t12178 / F::cast_from(768.0_f64);
    t54625
}
