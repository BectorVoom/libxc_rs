//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 787/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk787<F: Float>(t26: F, t41743: F, t2360: F, t322: F, t17: F, t197: F, t8991: F, t675: F, t9568: F, t683: F, t7514: F, t191: F, t33300: F, t2371: F, t2404: F, t27: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41744 = t26 * t41743;
    let t41745 = 280.0 / 81.0 * t41744;
    let t41751 = 1.0 / t322 / t2360;
    let t41752 = t17 * t41751;
    let t41768 = t8991 / t197;
    let t41816 = t9568 * t675;
    let t41825 = t683 * t7514;
    let t41848 = t191 * t33300;
    let t41879 = t2404 * t2371;
    let t41911 = t27 * t41751;
    (t41744, t41745, t41751, t41752, t41768, t41816, t41825, t41848, t41879, t41911)
}
