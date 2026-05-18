//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 851/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk851<F: Float>(t33340: F, t33459: F, t33475: F, t35351: F, t35356: F, t35519: F, t35523: F, t35527: F, t35531: F, t35535: F, t35539: F, t35543: F) -> F {
    let t35677 = t33340 + t35351 / F::new(6.0) + t35356 - t35519 / F::new(2.0) - t33459 - F::new(2.0) / F::new(3.0) * t35523 - F::new(6.0) * t35527 + F::new(4.0) * t35531 + t33475 + t35535 / F::new(3.0) + F::new(2.0) * t35539 - t35543;
    t35677
}
