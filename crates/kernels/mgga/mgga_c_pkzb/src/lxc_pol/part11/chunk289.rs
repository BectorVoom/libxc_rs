//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 289/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk289<F: Float>(t23: F, t28: F, t7: F, t974: F, t980: F, t984: F) -> F {
    let t987 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7 * t974 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t980 * t28 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23 * t984;
    t987
}
