//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1382/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1382<F: Float>(t55487: F, t55491: F, t55500: F, t55508: F, t56978: F, t56980: F, t56982: F, t56984: F, t56986: F, t56988: F, t56990: F, t56992: F, t56994: F) -> F {
    let t58655 = -t55487 + t56978 / F::new(48.0) + t56980 / F::new(12.0) - F::new(7.0) / F::new(36.0) * t56982 + F::new(5.0) / F::new(192.0) * t56984 - t55491 + t55500 + t56986 / F::new(384.0) - F::new(7.0) / F::new(576.0) * t56988 + t55508 - F::new(7.0) / F::new(72.0) * t56990 + F::new(7.0) / F::new(36.0) * t56992 + t56994 / F::new(48.0);
    t58655
}
