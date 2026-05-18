//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1284/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1284<F: Float>(t14425: F, t51563: F, t13917: F, t53156: F, t9569: F, t14657: F, t51511: F, t51179: F, t14765: F, t6472: F, t833: F, t8782: F) -> (F, F, F, F, F) {
    let t53873 = t51563 * t14425;
    let t53874 = F::new(7.0) / F::new(1152.0) * t53873;
    let t53876 = t13917 * t53156 * t9569;
    let t53878 = t14657 * t51511;
    let t53880 = t14657 * t51179;
    let t53884 = t8782 * t6472 * t14765 * t833;
    (t53874, t53876, t53878, t53880, t53884)
}
