//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 788/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk788<F: Float>(t6466: F, t6474: F, t6477: F, t6482: F, t6486: F, t6490: F, t6495: F, t6497: F, t6502: F, t6506: F, t6508: F, t6511: F, t6513: F, t902: F, t914: F, t929: F) -> F {
    let t6516 = t902 * t6466 / F::new(1536.0) + t902 * t6474 / F::new(384.0) - F::new(7.0) / F::new(384.0) * t6477 - t6482 - t6486 + t6490 + t6495 - t914 * t6497 / F::new(1536.0) - F::new(7.0) / F::new(256.0) * t6502 - F::new(119.0) / F::new(1152.0) * t6506 + F::new(7.0) / F::new(384.0) * t6508 - t6511 - t929 * t6513 / F::new(768.0);
    t6516
}
