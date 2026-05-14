//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 854/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk854<F: Float>(t34333: F, t6210: F, t458: F, t7580: F, t6219: F, t25462: F, t34008: F, t25162: F, t33870: F, t25026: F, t631: F) -> (F, F, F, F, F, F) {
    let t143008 = t6210 * t34333;
    let t143017 = t7580 * t458;
    let t143018 = t143017 * t6219;
    let t143024 = t25462 * t34008;
    let t143038 = t25162 * t33870;
    let t143040 = t25026 * t631;
    (t143008, t143017, t143018, t143024, t143038, t143040)
}
