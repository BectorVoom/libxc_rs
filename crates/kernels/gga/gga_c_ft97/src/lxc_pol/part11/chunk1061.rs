//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1061/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1061<F: Float>(t10121: F, t2568: F, t766: F, t41433: F, t41437: F, t41439: F, t41443: F, t41797: F, t41800: F, t41803: F, t41806: F, t41808: F, t41810: F, t41812: F, t41814: F, t41819: F, t41823: F, t41829: F) -> (F, F) {
    let t41990 = t2568 * t766 * t10121;
    let t42009 = -F::new(8.0) / F::new(3.0) * t41433 + F::new(8.0) / F::new(9.0) * t41437 + F::new(8.0) / F::new(9.0) * t41439 - F::new(8.0) / F::new(3.0) * t41443 - t41797 / F::new(3.0) - F::new(8.0) / F::new(9.0) * t41800 + F::new(4.0) / F::new(9.0) * t41803 + F::new(112.0) / F::new(81.0) * t41806 - F::new(4.0) / F::new(9.0) * t41808 - F::new(8.0) / F::new(9.0) * t41810 + F::new(8.0) / F::new(27.0) * t41812 - F::new(8.0) / F::new(27.0) * t41814 + F::new(40.0) / F::new(81.0) * t41819 - F::new(20.0) / F::new(27.0) * t41823 + F::new(8.0) / F::new(3.0) * t41829;
    (t41990, t42009)
}
