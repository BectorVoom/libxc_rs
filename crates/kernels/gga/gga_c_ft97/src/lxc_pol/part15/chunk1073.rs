//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1073/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1073<F: Float>(t49266: F, t62246: F, t77914: F, t77917: F, t77920: F, t77935: F, t77990: F, t86986: F, t86989: F, t86992: F, t86995: F, t86998: F, t87002: F, t87006: F, t87011: F) -> F {
    let t87128 = F::new(8.0) * t77914 + F::new(8.0) / F::new(3.0) * t77917 + F::new(40.0) / F::new(81.0) * t77920 - t86986 + F::new(8.0) / F::new(3.0) * t86989 - F::new(8.0) / F::new(9.0) * t86992 + F::new(4.0) / F::new(3.0) * t86995 - F::new(12.0) * t86998 + F::new(6.0) * t87002 - F::new(3.0) / F::new(4.0) * t87006 + F::new(4.0) / F::new(3.0) * t77935 - F::new(8.0) * t87011 - F::new(8.0) / F::new(3.0) * t62246 + F::new(112.0) / F::new(27.0) * t49266 - F::new(8.0) / F::new(3.0) * t77990;
    t87128
}
