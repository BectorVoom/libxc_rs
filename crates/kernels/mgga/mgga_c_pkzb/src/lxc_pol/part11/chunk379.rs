//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 379/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk379<F: Float>(t501: F, t546: F, t496: F, t513: F, t465: F, t519: F, t106: F, t518: F, t101: F, t525: F) -> (F, F, F, F, F, F, F, F) {
    let t1555 = 8.0 * t501 * t546;
    let t1556 = t496 * t513;
    let t1559 = 8.0 * t496 * t546;
    let t1564 = t465 * t519;
    let t1568 = t518 * t106;
    let t1569 = 1.0 / t1568;
    let t1570 = t101 * t1569;
    let t1571 = t525 * t525;
    (t1555, t1556, t1559, t1564, t1568, t1569, t1570, t1571)
}
