//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 912/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk912<F: Float>(t1587: F, t1852: F, t971: F, t1045: F, t526: F, t1985: F, t2179: F, t1613: F, t1689: F, t1326: F, t8417: F, t1851: F, t5704: F) -> (F, F, F, F, F, F, F, F) {
    let t60426 = t1587 * t1852;
    let t60901 = t1587 * t971;
    let t63180 = t526 * t1045;
    let t63755 = t1985 * t2179;
    let t64242 = t1985 * t1045;
    let t79931 = t1689 * t1613;
    let t91493 = t1326 * t8417;
    let t91496 = t5704 * t1851;
    (t60426, t60901, t63180, t63755, t64242, t79931, t91493, t91496)
}
