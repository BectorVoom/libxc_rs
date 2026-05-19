//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1360/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1360<F: Float>(t35662: F, t35664: F, t35638: F, t36421: F, t36422: F, t36423: F, t36425: F, t36426: F, t36427: F, t36428: F, t36429: F, t36430: F, t36431: F) -> F {
    let t36432 = F::cast_from(0.809822844183586641e-4_f64) * t35662;
    let t36433 = F::cast_from(0.28073858598364336888e-2_f64) * t35664;
    let t36434 = t36421 + t36422 + t36423 + F::cast_from(0.54311401758461002391e-5_f64) * t35638 + t36425 + t36426 - t36427 - t36428 + t36429 + t36430 - t36431 + t36432 + t36433;
    t36434
}
