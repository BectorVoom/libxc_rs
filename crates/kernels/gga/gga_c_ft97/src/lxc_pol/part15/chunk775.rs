//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 775/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk775<F: Float>(t2: F, t37355: F, t102: F, t8416: F, t100: F, t37292: F, t24: F, t32075: F, t1636: F, t443: F, t444: F) -> (F, F, F, F, F) {
    let t38571 = t2 * t37355;
    let t38651 = 1.0 / t8416 / t102;
    let t38652 = t100 * t38651;
    let t38771 = 280.0 / 243.0 * t37292;
    let t38921 = t24 * t32075;
    let t38953 = t443 * t444 * t1636;
    (t38571, t38652, t38771, t38921, t38953)
}
