//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 891/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk891<F: Float>(t3021: F, t8577: F, t3016: F, t385: F, t375: F, t3020: F, t8561: F, t1051: F, t8434: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t8579 = F::cast_from(0.48245472966453314466e2_f64) * t8577 * t3021;
    let t8581 = F::new(1.0) / t3016 / t385;
    let t8582 = t375 * t8581;
    let t8583 = t8561 * t3020;
    let t8585 = F::cast_from(0.96490945932906628932e2_f64) * t8582 * t8583;
    let t8586 = t1051 * t8434;
    let t8587 = t25 * t8586;
    (t8579, t8581, t8582, t8583, t8585, t8586, t8587)
}
