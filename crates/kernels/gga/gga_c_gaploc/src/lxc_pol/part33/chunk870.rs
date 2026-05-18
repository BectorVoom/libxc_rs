//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 870/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk870<F: Float>(t1628: F, t3060: F, t1589: F, t2963: F, t7088: F, t7090: F, t7093: F, t7096: F, t7098: F, t7100: F, t1020: F, t2919: F, t471: F, t64: F, t90: F) -> (F, F, F) {
    let t8693 = t1628 * t3060;
    let t8696 = t1589 * t2963;
    let t8710 = F::new(189.0) / F::new(256.0) * t7088 - F::new(483.0) / F::new(8192.0) * t7090 + F::new(147.0) / F::new(524288.0) * t7093 - F::new(49.0) / F::new(524288.0) * t7096 + F::new(161.0) / F::new(8192.0) * t7098 - F::new(63.0) / F::new(256.0) * t7100;
    let t8720 = t8710 * t471 - F::new(8.0) / F::new(3.0) * t2919 * t64 + F::new(4.0) / F::new(3.0) * t1020 * t90 + F::new(63.0) / F::new(256.0) * t7088 - F::new(49.0) / F::new(8192.0) * t7090 + F::new(49.0) / F::new(24576.0) * t7098 - F::new(21.0) / F::new(256.0) * t7100;
    (t8693, t8696, t8720)
}
