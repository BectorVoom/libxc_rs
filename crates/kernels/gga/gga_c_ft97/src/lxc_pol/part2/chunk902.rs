//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 902/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk902<F: Float>(t13722: F, t13700: F, t13704: F, t13708: F, t13719: F, t9701: F, t9735: F, t9861: F, t9862: F, t9869: F, t9870: F, t13739: F) -> (F, F) {
    let t13976 = F::new(4.0) / F::new(27.0) * t13722;
    let t13977 = t13700 / F::new(2.0) - F::new(4.0) / F::new(3.0) * t13704 + F::new(4.0) / F::new(9.0) * t13708 + t9861 + t9862 - F::new(8.0) / F::new(27.0) * t9735 - F::new(8.0) / F::new(9.0) * t9701 - t9869 + t9870 - F::new(6.0) * t13719 - t13976;
    let t13981 = F::new(4.0) / F::new(9.0) * t13739;
    (t13977, t13981)
}
