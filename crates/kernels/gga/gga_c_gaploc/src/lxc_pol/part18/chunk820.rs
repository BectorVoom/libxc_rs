//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 820/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk820<F: Float>(t6363: F, t6366: F, t6374: F, t6377: F, t6379: F, t6381: F, t2748: F, t471: F, t64: F, t90: F, t984: F) -> (F, F) {
    let t7851 = F::cast_from(189.0_f64) / F::cast_from(256.0_f64) * t6363 - F::cast_from(483.0_f64) / F::cast_from(8192.0_f64) * t6366 + F::cast_from(147.0_f64) / F::cast_from(524288.0_f64) * t6374 - F::cast_from(49.0_f64) / F::cast_from(524288.0_f64) * t6377 + F::cast_from(161.0_f64) / F::cast_from(8192.0_f64) * t6379 - F::cast_from(63.0_f64) / F::cast_from(256.0_f64) * t6381;
    let t7861 = t7851 * t471 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2748 * t64 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t984 * t90 + F::cast_from(63.0_f64) / F::cast_from(256.0_f64) * t6363 - F::cast_from(49.0_f64) / F::cast_from(8192.0_f64) * t6366 + F::cast_from(49.0_f64) / F::cast_from(24576.0_f64) * t6379 - F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t6381;
    (t7851, t7861)
}
