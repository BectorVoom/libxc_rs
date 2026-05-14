//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1240/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1240<F: Float>(t30979: F, t96925: F, t10157: F, t27787: F, t3837: F, t6118: F, t24543: F, t30967: F, t24437: F, t2574: F, t27796: F, t108140: F, t110067: F, t110068: F, t110069: F, t110077: F, t110085: F, t123835: F, t123840: F) -> (F, F, F, F, F) {
    let t123842 = t96925 * t30979;
    let t123846 = t6118 * t10157 * t27787 * t3837;
    let t123849 = t24543 * t30967;
    let t123853 = t24437 * t2574 * t27787 * t27796;
    let t123855 = -t123835 / 54.0 - t123840 / 36.0 - t110067 - t110068 - t110069 - t110077 + t123842 / 54.0 - 2.0 * t123846 - t110085 + 8.0 / 81.0 * t108140 - t123849 / 81.0 - t123853 / 3.0;
    (t123842, t123846, t123849, t123853, t123855)
}
