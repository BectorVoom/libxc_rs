//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1328/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1328<F: Float>(t53806: F, t14902: F, t9270: F, t14928: F, t840: F, t2409: F, t2410: F, t3066: F, t36129: F, t36200: F, t36201: F, t4097: F, t4207: F, t51719: F, t51724: F, t52473: F, t53775: F, t53804: F, t53809: F, t53811: F, t53816: F, t53832: F, t53843: F) -> F {
    let t55375 = F::new(7.0) / F::new(12.0) * t53806;
    let t55382 = F::new(7.0) / F::new(72.0) * t9270 * t14902;
    let t55385 = F::new(7.0) / F::new(144.0) * t840 * t14928;
    let t55392 = t36200 * t36201 * t4207 * t2410 / F::new(4.0) - t53775 / F::new(24.0) + t53804 / F::new(384.0) - t55375 + F::new(7.0) / F::new(144.0) * t51719 + t53809 / F::new(8.0) + t53811 / F::new(4.0) - F::new(7.0) / F::new(72.0) * t51724 - t53816 / F::new(384.0) - t55382 - t53832 / F::new(2.0) + t55385 - F::new(7.0) / F::new(72.0) * t52473 + t3066 * t2409 * t36129 * t4097 / F::new(24.0) - t53843 / F::new(4.0);
    t55392
}
