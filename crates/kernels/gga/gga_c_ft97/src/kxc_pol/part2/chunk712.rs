//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 712/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk712<F: Float>(t1710: F, t3099: F, t371: F, t7876: F, t1630: F, t929: F, t25: F, t78: F, t1602: F, t122: F, t1593: F, t1664: F, t939: F) -> (F, F, F, F, F, F, F) {
    let t11225 = t1710 * t3099;
    let t11232 = t371 * t7876;
    let t11233 = t1630 * t929;
    let t11240 = t78 * t25;
    let t11241 = t1602 * t11240;
    let t11245 = t78 * t122;
    let t11246 = t1602 * t11245;
    let t11247 = t1593 * t929;
    let t11251 = t1664 * t939;
    (t11225, t11232, t11233, t11241, t11246, t11247, t11251)
}
