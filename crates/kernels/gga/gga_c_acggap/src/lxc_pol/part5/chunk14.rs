//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 14/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk14<F: Float>(t21: F, t22: F, t5: F, t11: F, t14: F, t17: F) -> (F, F, F, F) {
    let t25 = t21 * t5 / t22;
    let t27 = F::cast_from(0.379785e1_f64) * t14 + F::cast_from(0.8969e0_f64) * t11 + F::cast_from(0.204775e0_f64) * t17 + F::cast_from(0.123235e0_f64) * t25;
    let t30 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t27;
    let t31 = F::ln(t30);
    (t25, t27, t30, t31)
}
