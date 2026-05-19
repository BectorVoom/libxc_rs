//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 512/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk512<F: Float>(t1009: F, t2554: F, t2473: F, t2529: F, t837: F, t845: F, t1000: F, t2270: F, t914: F) -> (F, F, F, F, F) {
    let t2555 = t2554 * t1009;
    let t2559 = t2529 * t2473 * t837;
    let t2561 = F::cast_from(0.11696446794910408142e1_f64) * t845 * t2559;
    let t2562 = t1000 * t2270;
    let t2563 = t914 * t2562;
    (t2555, t2559, t2561, t2562, t2563)
}
