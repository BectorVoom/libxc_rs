//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 921/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk921<F: Float>(t100775: F, t363: F, t22511: F, t3076: F, t32167: F, t5555: F, t938: F, t47: F, t8: F, t11119: F, t92642: F, t497: F, t6454: F) -> (F, F, F, F, F, F) {
    let t100776 = t100775 * t363;
    let t101075 = t3076 * t32167 * t22511;
    let t101161 = t5555 * t938;
    let t101248 = t8 * t47;
    let t101507 = t11119 * t92642;
    let t101975 = t6454 * t497;
    (t100776, t101075, t101161, t101248, t101507, t101975)
}
