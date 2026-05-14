//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 871/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk871<F: Float>(t33068: F, t8392: F, t32992: F, t604: F, t33036: F, t2101: F, t7390: F, t7312: F, t358: F, t7407: F, t33204: F, t33200: F, t7400: F, t9438: F, t7354: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t139634 = t8392 * t33068;
    let t139661 = t604 * t32992;
    let t139666 = t8392 * t33036;
    let t139675 = t2101 * t7390;
    let t139679 = t604 * t7312;
    let t139702 = t7407 * t358;
    let t139716 = t8392 * t33204;
    let t139722 = t8392 * t33200;
    let t139757 = t9438 * t7400;
    let t139767 = 8.0 / 27.0 * t8232 * t7354;
    (t139634, t139661, t139666, t139675, t139679, t139702, t139716, t139722, t139757, t139767)
}
