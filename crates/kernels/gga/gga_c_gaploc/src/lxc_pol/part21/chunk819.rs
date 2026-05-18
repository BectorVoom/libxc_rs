//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 819/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk819<F: Float>(t6363: F, t6366: F, t6374: F, t6377: F, t6379: F, t6381: F, t2748: F, t471: F, t64: F, t90: F, t984: F) -> F {
    let t7851 = F::new(189.0) / F::new(256.0) * t6363 - F::new(483.0) / F::new(8192.0) * t6366 + F::new(147.0) / F::new(524288.0) * t6374 - F::new(49.0) / F::new(524288.0) * t6377 + F::new(161.0) / F::new(8192.0) * t6379 - F::new(63.0) / F::new(256.0) * t6381;
    let t7861 = t7851 * t471 - F::new(8.0) / F::new(3.0) * t2748 * t64 + F::new(4.0) / F::new(3.0) * t984 * t90 + F::new(63.0) / F::new(256.0) * t6363 - F::new(49.0) / F::new(8192.0) * t6366 + F::new(49.0) / F::new(24576.0) * t6379 - F::new(21.0) / F::new(256.0) * t6381;
    t7861
}
