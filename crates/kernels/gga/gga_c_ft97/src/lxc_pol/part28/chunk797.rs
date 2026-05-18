//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 797/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk797<F: Float>(t139: F, t527: F, t52: F, t538: F, t7182: F, t1995: F, t32311: F, t7335: F, t71: F, t420: F, t7195: F, t135: F, t32318: F) -> (F, F, F, F, F, F, F) {
    let t32815 = t527 * t139;
    let t32817 = t52 * t7182 * t538;
    let t32822 = t1995 * t139;
    let t32836 = F::new(0.26675734978222673832e-1) * t7335 * t32311;
    let t32837 = t71 * t538;
    let t32838 = t420 * t32837;
    let t32839 = t7195 * t32838;
    let t32848 = t32318 * t135;
    (t32815, t32817, t32822, t32836, t32837, t32839, t32848)
}
