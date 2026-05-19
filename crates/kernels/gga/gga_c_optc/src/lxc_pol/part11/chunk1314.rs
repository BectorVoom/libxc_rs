//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1314/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1314<F: Float>(t39565: F, t49404: F, t49406: F, t57057: F, t57060: F, t57063: F, t57066: F, t57069: F, t57071: F, t57073: F, t57100: F, t57102: F, t57104: F, t57106: F) -> F {
    let t57447 = F::cast_from(0.13892666666666666667e1_f64) * t39565 + F::new(0.166712e1) * t49404 - F::cast_from(0.55570666666666666668e0_f64) * t49406 - F::new(0.516475e0) * t57057 + F::new(0.123954e2) * t57060 + F::new(0.309885e1) * t57063 + F::new(0.94674375e0) * t57066 + F::cast_from(0.2366859375e0_f64) * t57069 - F::new(0.52945875e1) * t57071 - F::cast_from(0.6618234375e1_f64) * t57073 + F::new(0.6311625e0) * t57100 + F::cast_from(0.158837625e2_f64) * t57102 - F::new(0.705945e1) * t57104 + F::new(0.3529725e1) * t57106;
    t57447
}
