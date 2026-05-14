//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 963/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk963<F: Float>(t1384: F, t4805: F, t2179: F, t26567: F, t925: F, t1969: F, t26783: F, t4462: F, t5773: F, t4454: F, t9049: F, t2: F, t20526: F, t4: F, t26: F, t1360: F, t4837: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30133 = t1384 * t4805;
    let t30134 = t2179 * t30133;
    let t30136 = t26567 * t925;
    let t30137 = t1969 * t30136;
    let t30141 = t1969 * t26783 * t925;
    let t30145 = t1969 * t5773 * t4462;
    let t30149 = t9049 * t5773 * t4454;
    let t30154 = t20526 * t2;
    let t30155 = t30154 * t4;
    let t30156 = t30155 * t26;
    let t30161 = t1360 * t4837;
    (t30133, t30134, t30137, t30141, t30145, t30149, t30155, t30156, t30161)
}
