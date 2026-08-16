//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 163/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk163<F: Float>(t373: F, t376: F, t379: F, t383: F) -> (F, F, F) {
    let t385 = F::cast_from(0.379785e1_f64) * t376 + F::cast_from(0.8969e0_f64) * t373 + F::cast_from(0.204775e0_f64) * t379 + F::cast_from(0.123235e0_f64) * t383;
    let t388 = F::cast_from(1.0_f64) + F::cast_from(0.16081824322151104822e2_f64) / t385;
    let t389 = F::ln(t388);
    (t385, t388, t389)
}
