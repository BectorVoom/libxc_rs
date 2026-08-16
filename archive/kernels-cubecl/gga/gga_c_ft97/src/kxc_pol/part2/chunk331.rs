//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 331/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk331<F: Float>(t1655: F, t35: F, t374: F, t1594: F, t1632: F, t401: F, t38: F, t78: F, t388: F, t66: F, t408: F, t428: F) -> (F, F, F, F, F, F, F) {
    let t1656 = t1655 * t35;
    let t1657 = t374 * t1656;
    let t1660 = t1594 * t1632;
    let t1663 = t401 * t401;
    let t1664 = t38 * t1663;
    let t1665 = t1664 * t78;
    let t1669 = t388 * t66;
    let t1670 = t408 * t401;
    let t1671 = t1670 * t428;
    (t1657, t1660, t1663, t1664, t1665, t1669, t1671)
}
