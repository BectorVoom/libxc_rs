//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 622/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk622<F: Float>(t24485: F, t24500: F, t24517: F, t24524: F, t24628: F, t27765: F, t27769: F, t27773: F, t27778: F, t27783: F, t27790: F, t27792: F) -> F {
    let t28069 = t27765 / F::new(27.0) - t27769 / F::new(9.0) - t27773 / F::new(36.0) - t27778 / F::new(36.0) - t24628 + t24485 / F::new(9.0) - t27783 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t24500 + t24517 / F::new(18.0) - t24524 / F::new(27.0) + t27790 / F::new(18.0) - t27792 / F::new(54.0);
    t28069
}
