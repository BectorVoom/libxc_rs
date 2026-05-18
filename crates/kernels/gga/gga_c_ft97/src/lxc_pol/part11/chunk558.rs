//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 558/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk558<F: Float>(t64: F, t7866: F, t2037: F, t428: F, t25: F, t409: F, t1602: F) -> (F, F, F, F) {
    let t7867 = t64 * t7866;
    let t7868 = t2037 * t428;
    let t7876 = t409 * t25;
    let t7877 = t1602 * t7876;
    (t7867, t7868, t7876, t7877)
}
