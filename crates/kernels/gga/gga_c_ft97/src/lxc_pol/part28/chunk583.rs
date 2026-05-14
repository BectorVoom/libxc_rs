//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 583/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk583<F: Float>(t25978: F, t446: F, t22953: F, t379: F, t6495: F, t22952: F, t473: F, t920: F, t5691: F, t432: F, t6454: F) -> (F, F, F, F, F) {
    let t25979 = t446 * t25978;
    let t25982 = t22953 * t6495 * t379;
    let t25983 = t22952 * t25982;
    let t25985 = t920 * t473;
    let t25987 = t22953 * t5691 * t25985;
    let t25988 = t22952 * t25987;
    let t25990 = t6454 * t432;
    (t25979, t25983, t25985, t25988, t25990)
}
