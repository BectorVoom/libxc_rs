//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 445/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk445<F: Float>(t1969: F, t4656: F, t446: F, t1974: F, t4417: F, t356: F, t89: F, t4431: F, t519: F, t1017: F) -> (F, F, F, F, F, F, F) {
    let t4657 = t1969 * t4656;
    let t4658 = t446 * t4657;
    let t4660 = t1974 * t4417;
    let t4662 = t89 * t356 * t4660;
    let t4664 = t519 * t4431;
    let t4666 = t89 * t356 * t4664;
    let t4668 = t1017 * t1017;
    (t4657, t4658, t4660, t4662, t4664, t4666, t4668)
}
