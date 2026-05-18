//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 306/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk306<F: Float>(t22: F, t28: F, t34: F, t38: F, t974: F, t984: F, tau1: F) -> (F, F) {
    let t991 = tau1 * t22;
    let t995 = -F::new(5.0) / F::new(3.0) * t991 * t28 + F::new(5.0) / F::new(3.0) * t34 * t974 + F::new(5.0) / F::new(3.0) * t38 * t984;
    (t991, t995)
}
