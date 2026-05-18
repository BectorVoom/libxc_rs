//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 973/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk973<F: Float>(t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t17399: F, t17401: F, t17403: F, t17406: F, t17409: F, t17412: F, t17419: F, t8832: F) -> F {
    let t17833 = -t8832 - F::new(0.103295e1) * t17346 + F::new(0.309885e1) * t17354 - F::new(0.52945875e1) * t17399 + F::new(0.94674375e0) * t17401 + F::new(0.6311625e0) * t17403 + F::new(0.20839e0) * t17406 - F::new(0.62517e0) * t17409 - F::new(0.46308888888888888889e-1) * t17412 - F::new(0.57386111111111111112e0) * t17338 + F::new(0.20659e1) * t17342 - F::new(0.309885e1) * t17350 - F::new(0.516475e0) * t17358 - F::new(0.104195e0) * t17419;
    t17833
}
