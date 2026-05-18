//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 848/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk848<F: Float>(t184: F, t363: F, t3663: F, t1078: F, t2299: F, t3664: F, t2300: F, t920: F, t1079: F, t1580: F, t3596: F, t5: F) -> (F, F, F, F, F) {
    let t13255 = t184 * t363;
    let t13256 = t3663 * t13255;
    let t13259 = t1078 * t2299;
    let t13260 = t13259 * t3664;
    let t13263 = t2300 * t920;
    let t13268 = t1079 * t1580;
    let t13273 = t5 * t3596;
    (t13256, t13260, t13263, t13268, t13273)
}
