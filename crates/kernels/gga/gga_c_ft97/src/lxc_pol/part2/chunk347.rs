//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 347/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk347<F: Float>(t1565: F, t1787: F, t1570: F, t2: F, t1559: F, t463: F, t1580: F, t464: F, t1586: F, t1588: F, t24: F, t1755: F, t469: F) -> (F, F, F, F, F, F, F, F) {
    let t1788 = t1787 * t1565;
    let t1791 = t2 * t1570;
    let t1792 = t1791 * t1559;
    let t1793 = t463 * t1792;
    let t1796 = t464 * t1580;
    let t1797 = t463 * t1796;
    let t1800 = t1586 * t2;
    let t1802 = t24 * t1800 * t1588;
    let t1806 = t24 * t469 * t1755;
    (t1788, t1792, t1793, t1796, t1797, t1800, t1802, t1806)
}
