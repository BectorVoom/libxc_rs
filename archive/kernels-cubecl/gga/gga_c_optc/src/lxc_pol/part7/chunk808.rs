//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 808/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk808<F: Float>(t7525: F, t7527: F, t7531: F, t7535: F, t7541: F, t7547: F, t7550: F, t7576: F, t7580: F, t7583: F, t7593: F, t7594: F, t7596: F, t7599: F) -> F {
    let t7603 = F::cast_from(0.5519e-1_f64) * t7576 - F::cast_from(0.36793333333333333333e-1_f64) * t7580 - F::cast_from(0.82785e-1_f64) * t7583 - F::cast_from(0.40256666666666666668e0_f64) * t7525 + F::cast_from(0.30192500000000000001e0_f64) * t7531 + F::cast_from(0.20128333333333333333e0_f64) * t7527 - F::cast_from(0.33547222222222222222e0_f64) * t7535 - F::cast_from(0.301925e0_f64) * t7550 - t7593 - t7594 - F::cast_from(0.82785e-1_f64) * t7596 + F::cast_from(0.49671e0_f64) * t7599 - F::cast_from(0.60384999999999999999e0_f64) * t7541 + F::cast_from(0.181155e1_f64) * t7547;
    t7603
}
