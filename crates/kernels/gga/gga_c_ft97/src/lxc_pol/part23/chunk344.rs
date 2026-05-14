//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 344/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk344<F: Float>(t3821: F, t676: F, t27: F, t89: F, t2335: F, t2338: F, t2341: F, t3688: F, t3693: F, t3697: F, t3702: F, t3707: F, t3710: F, t3715: F, t3720: F, t661: F) -> (F, F, F, F) {
    let t3822 = t676 * t3821;
    let t3824 = t89 * t27 * t3822;
    let t3826 = t2335 + t2338 / 54.0 + t2341 / 18.0 + t3688 / 54.0 - t3693 / 27.0 + t3697 / 18.0 + t3702 / 9.0 + t3707 / 9.0 + t3710 / 18.0 + t3715 / 18.0 + t3720 / 3.0 - t3824 / 6.0;
    let t3827 = t661 * t3826;
    (t3822, t3824, t3826, t3827)
}
