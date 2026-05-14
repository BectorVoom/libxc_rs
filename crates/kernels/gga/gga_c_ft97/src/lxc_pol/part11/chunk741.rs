//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 741/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk741<F: Float>(t2: F, t8275: F, t1555: F, t26: F, t1557: F, t469: F, t356: F, t1570: F, t100: F, t1587: F, t1852: F, t463: F, t110: F, t8216: F, t103: F, t7763: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11690 = t8275 * t2;
    let t11755 = t26 * t1555;
    let t11756 = t469 * t1557;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11810 = t1587 * t100;
    let t11854 = t463 * t1852;
    let t11863 = t8216 * t110;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    (t11690, t11755, t11756, t11761, t11762, t11810, t11854, t11863, t11987, t11988)
}
