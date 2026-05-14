//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 812/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk812<F: Float>(t62246: F, t62287: F, t62309: F, t62317: F, t4743: F, t8232: F, t4819: F, t38953: F, t4829: F, t4747: F, t4833: F, t4790: F, t582: F, t4739: F, t4807: F, t2101: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t62822 = 4.0 / 9.0 * t62246;
    let t62846 = 4.0 / 27.0 * t62287;
    let t62853 = 8.0 / 81.0 * t62309;
    let t62856 = 8.0 / 27.0 * t62317;
    let t63120 = t8232 * t4743;
    let t63157 = t8232 * t4819;
    let t63187 = t38953 * t4829;
    let t63219 = t8232 * t4747;
    let t63225 = t8232 * t4833;
    let t63258 = t582 * t4790;
    let t63530 = t8232 * t4739;
    let t63536 = t8232 * t4807;
    let t63586 = t2101 * t4790;
    (t62822, t62846, t62853, t62856, t63120, t63157, t63187, t63219, t63225, t63258, t63530, t63536, t63586)
}
