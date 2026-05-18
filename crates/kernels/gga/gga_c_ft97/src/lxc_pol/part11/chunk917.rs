//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 917/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk917<F: Float>(t11587: F, t1647: F, t1901: F, t1909: F, t1922: F, t379: F, t38846: F, t38866: F, t446: F, t447: F, t487: F, t8206: F, t8212: F, t8355: F, t8372: F, t8387: F, t8425: F, t8506: F, t8510: F, t8511: F, t8519: F, t8535: F) -> F {
    let t38883 = -F::new(112.0) / F::new(81.0) * t38846 - F::new(4.0) / F::new(3.0) * t1901 * t1909 * t8510 * t1647 + F::new(4.0) / F::new(9.0) * t1901 * t1909 * t487 * t8355 * t379 + F::new(4.0) / F::new(3.0) * t1901 * t8506 * t8511 + F::new(8.0) / F::new(9.0) * t1901 * t11587 * t8212 + F::new(4.0) / F::new(3.0) * t1901 * t8372 * t8387 + F::new(8.0) / F::new(9.0) * t1901 * t38866 * t8519 - F::new(8.0) / F::new(3.0) * t1901 * t8506 * t8425 + F::new(8.0) / F::new(3.0) * t1901 * t8506 * t8535 - F::new(8.0) / F::new(3.0) * t1901 * t8372 * t8206 + F::new(4.0) / F::new(3.0) * t446 * t447 * t1922 * t1647;
    t38883
}
