//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 151/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk151<F: Float>(t16: F, t34: F, t38: F, t441: F, t445: F, t454: F) -> F {
    let t459 = -F::new(5.0) / F::new(3.0) * t454 * t16 + F::new(5.0) / F::new(3.0) * t34 * t441 + F::new(5.0) / F::new(3.0) * t38 * t445;
    t459
}
