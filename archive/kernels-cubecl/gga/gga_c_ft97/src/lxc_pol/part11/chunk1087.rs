//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1087/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1087<F: Float>(t41433: F, t41437: F, t41439: F, t41443: F, t41797: F, t41800: F, t41803: F, t41806: F, t41808: F, t41810: F, t41812: F, t41814: F, t41819: F, t41823: F, t41829: F) -> F {
    let t42724 = -F::cast_from(8.0_f64) * t41433 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41437 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41439 - F::cast_from(8.0_f64) * t41443 - t41797 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41800 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t41803 + F::cast_from(112.0_f64) / F::cast_from(27.0_f64) * t41806 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t41808 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t41810 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41812 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41814 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t41819 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t41823 + F::cast_from(8.0_f64) * t41829;
    t42724
}
