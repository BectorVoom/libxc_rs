//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1756/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1756<F: Float>(t43881: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F) -> F {
    let t90449 = -F::new(40.0) / F::new(81.0) * t81230 + F::new(16.0) / F::new(9.0) * t81232 - F::new(16.0) / F::new(27.0) * t68257 - F::new(8.0) / F::new(3.0) * t81234 - F::new(4.0) / F::new(9.0) * t81236 + F::new(40.0) / F::new(9.0) * t89865 - F::new(8.0) * t89869 + F::new(8.0) * t89873 + t89877 / F::new(3.0) - F::new(112.0) / F::new(81.0) * t56236 + t43881 + F::new(16.0) / F::new(9.0) * t68399;
    t90449
}
