//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1146/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1146<F: Float>(t139377: F, t139380: F, t148492: F, t148496: F, t148499: F, t148502: F, t148508: F, t148511: F, t148515: F, t148520: F, t148523: F, t148527: F, t148530: F, t148533: F, t148536: F, t148540: F) -> F {
    let t148541 = -F::new(2.0) / F::new(9.0) * t148492 + F::new(8.0) * t148496 + t148499 / F::new(18.0) + t148502 / F::new(3.0) + t139377 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t139380 + F::new(2.0) / F::new(3.0) * t148508 + F::new(2.0) / F::new(3.0) * t148511 + F::new(4.0) / F::new(3.0) * t148515 + t148520 / F::new(2.0) - F::new(2.0) / F::new(9.0) * t148523 - t148527 + F::new(2.0) / F::new(3.0) * t148530 - t148533 / F::new(9.0) + t148536 / F::new(27.0) + t148540;
    t148541
}
