//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3078/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078<F: Float>(t43888: F, t56236: F, t58073: F, t58075: F, t58090: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> F {
    let t81397 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t68332 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t68334 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t68336 - t58073 + t58075 + F::cast_from(6.0_f64) * t81224 + t81228 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t81230 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t81232 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t81234 - t81236 / F::cast_from(9.0_f64) + t58090 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t56236 - t68389 / F::cast_from(3.0_f64) + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68399 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t81242 - F::cast_from(4.0_f64) * t81245 - F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t43888 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t68454 - F::cast_from(2.0_f64) * t68456;
    t81397
}
