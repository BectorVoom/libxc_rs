//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 603/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk603<F: Float>(t27789: F, t6118: F, t24543: F, t6884: F, t24483: F, t24485: F, t24500: F, t24517: F, t24524: F, t27765: F, t27769: F, t27773: F, t27778: F, t27783: F) -> (F, F, F) {
    let t27790 = t6118 * t27789;
    let t27792 = t24543 * t6884;
    let t27794 = t27765 / F::new(9.0) - t27769 / F::new(3.0) - t27773 / F::new(12.0) - t27778 / F::new(12.0) - t24483 + t24485 / F::new(3.0) - t27783 - F::new(2.0) / F::new(3.0) * t24500 + t24517 / F::new(6.0) - t24524 / F::new(9.0) + t27790 / F::new(6.0) - t27792 / F::new(18.0);
    (t27790, t27792, t27794)
}
