//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 218/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk218<F: Float>(t920: F, t943: F, t924: F, t935: F, t940: F, t947: F) -> (F, F, F) {
    let t964 = 0.301925e0 * t920;
    let t967 = 0.82785e-1 * t943;
    let t969 = 0.258925e1 * t935 - t964 - 0.301925e0 * t924 + 0.16504875e0 * t940 - t967 - 0.82785e-1 * t947;
    (t964, t967, t969)
}
