//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1397/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1397<F: Float>(t15532: F, t840: F, t52560: F, t55850: F, t55851: F, t55863: F, t55884: F, t57584: F, t57593: F, t57595: F, t57598: F, t57602: F, t57605: F, t57608: F, t57614: F, t57626: F, t57635: F) -> F {
    let t58875 = t840 * t15532;
    let t58883 = -t55850 + t55851 + t57584 / F::new(384.0) + t57593 / F::new(384.0) - F::new(35.0) / F::new(432.0) * t52560 + t57595 / F::new(12.0) - t57598 / F::new(24.0) + t55863 + F::new(7.0) / F::new(144.0) * t58875 - t57602 / F::new(192.0) - t57605 / F::new(24.0) - t57608 / F::new(48.0) + t57614 / F::new(8.0) - t57626 / F::new(384.0) + t55884 - t57635 / F::new(768.0);
    t58883
}
