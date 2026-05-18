//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 465/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk465<F: Float>(t4733: F, t574: F, t605: F, t1060: F, t569: F, t925: F, t167: F, t4462: F, t2205: F, t4454: F, t1039: F, t2086: F, t91: F) -> (F, F, F, F, F, F) {
    let t4735 = t574 * t605 * t4733;
    let t4739 = t569 * t1060 * t925;
    let t4743 = t569 * t167 * t4462;
    let t4747 = t2205 * t167 * t4454;
    let t4753 = t1039 * t1039;
    let t4755 = t91 * t2086 * t4753;
    (t4735, t4739, t4743, t4747, t4753, t4755)
}
