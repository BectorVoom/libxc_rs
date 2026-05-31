//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 905/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk905<F: Float>(t1995: F, t8851: F, t527: F, t23: F, t32905: F, t153: F, t1984: F, t22: F, t36452: F, t37991: F, t355: F, t7368: F) -> (F, F, F, F, F) {
    let t40087 = t1995 * t8851;
    let t40227 = t527 * t8851;
    let t40266 = t23 * t32905;
    let t40280 = F::cast_from(1.0_f64) / t153 / t37991 / t22 / t1984 / t36452 / F::cast_from(96.0_f64);
    let t40424 = t355 * t7368;
    (t40087, t40227, t40266, t40280, t40424)
}
