//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3078/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078<F: Float>(t43888: F, t56236: F, t58073: F, t58075: F, t58090: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> F {
    let t81397 = F::new(2.0) / F::new(9.0) * t68332 + F::new(4.0) / F::new(9.0) * t68334 + F::new(4.0) / F::new(3.0) * t68336 - t58073 + t58075 + F::new(6.0) * t81224 + t81228 / F::new(3.0) - F::new(10.0) / F::new(81.0) * t81230 + F::new(4.0) / F::new(9.0) * t81232 - F::new(2.0) / F::new(3.0) * t81234 - t81236 / F::new(9.0) + t58090 - F::new(28.0) / F::new(27.0) * t56236 - t68389 / F::new(3.0) + F::new(8.0) / F::new(9.0) * t68399 + F::new(10.0) / F::new(9.0) * t81242 - F::new(4.0) * t81245 - F::new(28.0) / F::new(81.0) * t43888 - F::new(4.0) / F::new(3.0) * t68454 - F::new(2.0) * t68456;
    t81397
}
