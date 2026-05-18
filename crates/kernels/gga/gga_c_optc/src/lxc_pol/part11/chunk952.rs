//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 952/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk952<F: Float>(t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t17399: F, t17401: F, t17403: F, t17406: F, t17409: F, t17412: F, t17419: F, t8728: F) -> F {
    let t17499 = -t8728 - F::new(0.60384999999999999999e0) * t17346 + F::new(0.181155e1) * t17354 - F::new(0.3883875e1) * t17399 + F::new(0.247573125e0) * t17401 + F::new(0.16504875e0) * t17403 + F::new(0.16557e0) * t17406 - F::new(0.49671e0) * t17409 - F::new(0.36793333333333333333e-1) * t17412 - F::new(0.33547222222222222222e0) * t17338 + F::new(0.12077e1) * t17342 - F::new(0.181155e1) * t17350 - F::new(0.301925e0) * t17358 - F::new(0.82785e-1) * t17419;
    t17499
}
