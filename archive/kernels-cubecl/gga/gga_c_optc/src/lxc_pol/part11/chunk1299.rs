//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1299/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1299<F: Float>(t39565: F, t49404: F, t49406: F, t57057: F, t57060: F, t57063: F, t57066: F, t57069: F, t57071: F, t57073: F, t57100: F, t57102: F, t57104: F, t57106: F) -> F {
    let t57179 = F::cast_from(0.11038e1_f64) * t39565 + F::cast_from(0.132456e1_f64) * t49404 - F::cast_from(0.44152e0_f64) * t49406 - F::cast_from(0.301925e0_f64) * t57057 + F::cast_from(0.72462e1_f64) * t57060 + F::cast_from(0.181155e1_f64) * t57063 + F::cast_from(0.247573125e0_f64) * t57066 + F::cast_from(0.6189328125e-1_f64) * t57069 - F::cast_from(0.3883875e1_f64) * t57071 - F::cast_from(0.485484375e1_f64) * t57073 + F::cast_from(0.16504875e0_f64) * t57100 + F::cast_from(0.11651625e2_f64) * t57102 - F::cast_from(0.51785e1_f64) * t57104 + F::cast_from(0.258925e1_f64) * t57106;
    t57179
}
