//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 378/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk378<F: Float>(t1266: F, t136: F, t191: F, t507: F, t604: F, t22: F, t643: F) -> (F, F, F, F) {
    let t1754 = t1266 * t136;
    let t1755 = t1754 * t191;
    let t1758 = t604 * t507;
    let t1762 = F::cast_from(1.0_f64) / t22 / t643;
    (t1754, t1755, t1758, t1762)
}
