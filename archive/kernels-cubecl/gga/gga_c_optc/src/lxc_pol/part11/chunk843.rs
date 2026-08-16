//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 843/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk843<F: Float>(t16323: F, t6: F, t6879: F, t161: F, t2024: F, t16220: F, t16249: F, t16251: F, t16252: F, t6318: F, t6321: F, t6324: F, t6328: F, t6330: F, t6356: F, t6526: F, t6619: F) -> (F, F, F, F, F, F) {
    let t16324 = t6 * t16323;
    let t16325 = t16324 * t6879;
    let t16326 = t161 * t16325;
    let t16329 = t16324 * t2024;
    let t16330 = t161 * t16329;
    let t16333 = -t6318 - t6321 - t6324 - t6328 - t6330 - t16220 + t6526 + t16249 - t6356 - t16251 - t6619 - t16252;
    (t16324, t16325, t16326, t16329, t16330, t16333)
}
