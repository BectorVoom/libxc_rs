//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 150/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk150<F: Float>(t6: F, t16: F, t34: F, t38: F, t441: F, t445: F, tau0: F) -> (F, F) {
    let t454 = tau0 * t6;
    let t459 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t454 * t16 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t34 * t441 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t38 * t445;
    (t454, t459)
}
