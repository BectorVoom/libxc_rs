//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 498/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk498<F: Float>(t1004: F, t1425: F, t1528: F, t4290: F, t4324: F, t4328: F, t436: F, t4361: F, t4365: F, t5372: F, t5464: F, t5471: F, t6004: F, t6006: F, t6007: F, t6008: F, t6009: F, t6010: F, t6011: F, t6012: F, t6013: F, t6014: F, t6067: F, t619: F) -> F {
    let t6301 = t6004 + t4290 - t6006 + t6007 - F::new(0.62182e-1) * t619 * t1004 * t1528 + t4361 - t4365 + t6008 + t6009 - t6010 + t4324 - t6011 + t4328 + F::new(0.93273e-1) * t436 * t6067 - t5464 + t6012 - t6013 + F::new(0.186546e0) * t1425 * t5372 + t5471 - t6014;
    t6301
}
