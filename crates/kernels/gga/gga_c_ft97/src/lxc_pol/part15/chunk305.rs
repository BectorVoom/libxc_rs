//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 305/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk305<F: Float>(t1570: F, t2: F, t1586: F, t1544: F, t102: F, t486: F) -> (F, F, F, F, F) {
    let t1791 = t2 * t1570;
    let t1800 = t1586 * t2;
    let t1812 = 4.0 / 9.0 * t1544;
    let t1832 = 4.0 / 27.0 * t1544;
    let t1851 = 1.0 / t486 / t102;
    (t1791, t1800, t1812, t1832, t1851)
}
