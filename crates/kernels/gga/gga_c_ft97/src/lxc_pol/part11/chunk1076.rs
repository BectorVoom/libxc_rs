//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1076/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1076<F: Float>(t42290: F, t42322: F, t761: F, t9974: F, t766: F, t2526: F, t2568: F, t762: F, t9895: F, t2492: F, t10015: F, t8392: F) -> (F, F, F, F, F, F) {
    let t42323 = t42290 + t42322;
    let t42328 = t9974 * t761;
    let t42329 = t42328 * t766;
    let t42331 = t2526 * t2526;
    let t42332 = t2568 * t42331;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    let t42344 = t8392 * t10015;
    (t42323, t42329, t42332, t42334, t42339, t42344)
}
