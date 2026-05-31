//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 578/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk578<F: Float>(t1923: F, t193: F, t195: F, t197: F, t3573: F, t4599: F, t4752: F, t750: F, t201: F, t5: F, t743: F) -> (F, F, F) {
    let t4756 = -t1923 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t3573 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t193 * t195 * t4599 * t197 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t193 * t750 * t4752;
    let t4758 = t5 * t4756 * t201;
    let t4759 = t743 * t4758;
    (t4756, t4758, t4759)
}
