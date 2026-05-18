//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1069/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1069<F: Float>(t17239: F, t4778: F, t91: F, t1969: F, t446: F, t86610: F, t86902: F, t40294: F, t7761: F, t85469: F, t89: F, t86665: F, t9073: F) -> (F, F, F, F, F) {
    let t87056 = t91 * t17239 * t4778;
    let t87060 = t446 * t1969 * t86610;
    let t87063 = t446 * t1969 * t86902;
    let t87067 = t89 * t7761 * t40294 * t85469;
    let t87071 = t446 * t9073 * t86665;
    (t87056, t87060, t87063, t87067, t87071)
}
