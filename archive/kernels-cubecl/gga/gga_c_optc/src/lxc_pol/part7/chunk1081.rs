//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1081/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1081<F: Float>(t6563: F, t740: F, t6602: F, t6607: F, t6763: F, t6766: F, t172: F, t1879: F, t22052: F, t22610: F, t22721: F, t22724: F, t22726: F, t22728: F, t3539: F, t606: F, t616: F, t6560: F, t95: F) -> F {
    let t23435 = t6563 * t740;
    let t23438 = F::cast_from(14.0_f64) / F::cast_from(3.0_f64) * t6602 * t740;
    let t23439 = t6607 * t740;
    let t23441 = t6763 * t6766;
    let t23452 = -F::cast_from(14.0_f64) / F::cast_from(3.0_f64) * t23435 - t23438 - F::cast_from(14.0_f64) * t23439 - t22721 + t22724 + F::cast_from(0.62027715443768233192e-1_f64) * t1879 * t23441 * t616 + F::cast_from(0.62027715443768233192e-1_f64) * t3539 * t172 * t6560 * t616 + t22726 + t22610 - t22728 + F::cast_from(0.77534644304710291488e-2_f64) * t95 * t606 * t22052;
    t23452
}
