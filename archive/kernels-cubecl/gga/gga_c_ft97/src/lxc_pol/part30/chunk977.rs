//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 977/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk977<F: Float>(t34053: F, t870: F, t34074: F, t8392: F, t34078: F, t34070: F, t34204: F, t7584: F, t10696: F, t7672: F, t7662: F, t848: F) -> (F, F, F, F, F, F, F, F) {
    let t143592 = t870 * t34053;
    let t143604 = t8392 * t34074;
    let t143606 = t8392 * t34078;
    let t143608 = t8392 * t34070;
    let t143610 = t8392 * t34204;
    let t143612 = t870 * t7584;
    let t143621 = t10696 * t7672;
    let t143653 = t848 * t7662;
    (t143592, t143604, t143606, t143608, t143610, t143612, t143621, t143653)
}
